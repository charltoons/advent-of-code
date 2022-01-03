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

#[tokio::main]
async fn main() -> Result<(), Error> {

    // Load environment variables from .env file
    dotenv::dotenv().ok();

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
    let depths: Vec<&str> = input.split("\n").collect();
    println!("Number of depths recorded: {}", depths.len());

    let mut previous_value = 0;
    let mut increases = 0;

    // Loop through the depths
    for i in depths {

        // When its a valid integer (last value is empty string)
        match i.parse::<i32>() {
            Ok(depth_int) => {

                // If this depth is greater than the immediately preceding value 
                // AND this isnt the first measurement.
                if depth_int > previous_value && previous_value > 0 {
                    increases = increases + 1;
                }

                // Set the next previous value
                previous_value = depth_int;
            },
            Err(_) => {}
        }
    }

    // Print the result
    println!("Number of depth increases: {}", increases);

    Ok(())
}
