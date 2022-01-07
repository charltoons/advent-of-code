use dotenv;
use std::env;
use reqwest::Error;

const ADVENT_CODE_SESSION_COOKIE_ENV_VAR: &str = "ADVENT_CODE_SESSION_COOKIE";

fn load_env_var(key: &str) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(_) => {
            panic!("Advent Code session cookie required as {} environment variable.", key);
        },
    }
}

#[derive(Debug)]
enum Instruction {
    Forward(usize),
    Down(usize),
    Up(usize),
}

async fn get_input() -> Result<Vec<Instruction>, Error> {
    // Load the session cookie variable
    let key = ADVENT_CODE_SESSION_COOKIE_ENV_VAR;
    let session_cookie = load_env_var(key);

    // Make the request for the input
    let request_url = format!(
        "https://adventofcode.com/2021/day/{day}/input",
        day = "2",
    );
    let client = reqwest::Client::new();
    let response = client.get(request_url)
        .header("cookie", format!("session={}", session_cookie))
        .send()
        .await?;
    let input = response.text().await?;

    // Get vector of depths
    let mut instructions = Vec::new();
    for instruction_str in input.split("\n").into_iter() {
        let kind = match instruction_str.split(" ").nth(0) {
            Some(kind) => kind,
            None => continue,
        };
        let value = match instruction_str.split(" ").nth(1) {
            Some(value) => match value.parse::<usize>() {
                Ok(value) => value,
                Err(_) => continue,
            },
            None => continue,
        };

        let instruction = match kind {
            "forward" => Instruction::Forward(value),
            "down" => Instruction::Down(value),
            "up" => Instruction::Up(value),
            _ => panic!("Unknown instruction kind: {}", kind),
        };

        instructions.push(instruction);
    }
    Ok(instructions)
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    // Load environment variables from .env file
    dotenv::dotenv().ok();

    let instructions: Vec<Instruction> = get_input().await?;
    println!("Number of instructions: {:?}", instructions.len());

    let mut position = 0;
    let mut depth = 0;

    for instruction in instructions.iter() {
        match instruction {
            Instruction::Forward(value) => {
                position += value;
            },
            Instruction::Down(value) => {
                depth += value;
            },
            Instruction::Up(value) => {
                depth -= value;
            },
        }
    }

    println!("Part 1 - Final position: {:?}", (position * depth));

    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for instruction in instructions.iter() {
        match instruction {
            Instruction::Forward(value) => {
                position += value;
                depth += aim * value;
            },
            Instruction::Down(value) => {
                aim += value;
            },
            Instruction::Up(value) => {
                aim -= value;
            },
        }
    }

    println!("Part 2 - Final position: {:?}", (position * depth));

    Ok(())
}
