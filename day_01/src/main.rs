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

fn part_1(depths: &Vec<u32>) -> u32 {
    let mut previous_value: Option<u32> = None;
    let mut increases = 0;

    // Loop through the depths
    for depth in depths {

        match previous_value {

            // On the first measurement, there is no previous value
            None => {},
            Some(prev) => {
                // If this depth is greater than the immediately preceding value 
                // AND this isnt the first measurement.
                if depth > &prev {
                    increases = increases + 1;
                }
            },
        }

        // Set the next previous value
        previous_value = Some(*depth);
    }
    increases
}

const WINDOW_SIZE: usize = 3;
fn part_2(depths: &Vec<u32>) -> u32 {
    let mut increases = 0;
    for (i, _) in depths.iter().enumerate() {

        // if we're past the final two windows to compare
        if (i + WINDOW_SIZE + 1) > depths.len() {
            break;
        }

        // Calculate the first window
        let mut window_a: u32 = 0;
        let start_idx = i;
        let end_idx = start_idx + WINDOW_SIZE;
        for j in start_idx..end_idx {
            window_a += depths[j];
        }

        // Calculate the second window
        let mut window_b: u32 = 0;
        let start_idx = i + 1;
        let end_idx = start_idx + WINDOW_SIZE;
        for j in start_idx..end_idx {
            window_b += depths[j];
        }

        // Record windowed depth increases
        if window_b > window_a {
            increases += 1;
        }
    }
    increases
}

async fn get_input() -> Result<Vec<u32>, Error> {
    // Load the session cookie variable
    let key = ADVENT_CODE_SESSION_COOKIE_ENV_VAR;
    let session_cookie = load_env_var(key);

    // Make the request for the input
    let request_url = format!(
        "https://adventofcode.com/2021/day/{day}/input",
        day = "1",
    );
    let client = reqwest::Client::new();
    let response = client.get(request_url)
        .header("cookie", format!("session={}", session_cookie))
        .send()
        .await?;
    let input = response.text().await?;

    // Get vector of depths
    let depths_str: Vec<&str> = input.split("\n").collect();
    let mut depths: Vec<u32> = Vec::new();
    for depth_str in depths_str.iter() {
        match depth_str.parse::<u32>() {
            Ok(depth) => depths.push(depth),
            Err(_) => {}
        };
    }
    Ok(depths)
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    // Load environment variables from .env file
    dotenv::dotenv().ok();

    let depths: Vec<u32> = get_input().await?;
    println!("Number of depths recorded: {}", depths.len());

    let increases = part_1(&depths);
    println!("Number of depth increases (part 1): {}", increases);

    let increases2 = part_2(&depths);
    println!("Number of depth increases (part 2): {}", increases2);

    Ok(())
}
