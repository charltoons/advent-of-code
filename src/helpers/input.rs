use reqwest::Error;
use std::env;

const ADVENT_CODE_SESSION_COOKIE_ENV_VAR: &str = "ADVENT_CODE_SESSION_COOKIE";

fn load_env_var(key: &str) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(_) => {
            panic!(
                "Advent Code session cookie required as {} environment variable.",
                key
            );
        }
    }
}

pub async fn fetch_input(day: usize) -> Result<String, Error> {
    // Load the session cookie variable
    let key = ADVENT_CODE_SESSION_COOKIE_ENV_VAR;
    let session_cookie = load_env_var(key);

    // Make the request for the input
    let request_url = format!("https://adventofcode.com/2021/day/{day}/input", day = day,);
    let client = reqwest::Client::new();
    let response = client
        .get(request_url)
        .header("cookie", format!("session={}", session_cookie))
        .send()
        .await?;
    let input = response.text().await?;
    Ok(input)
}

pub fn lines_to_vec(input: String) -> Vec<String> {
    let mut lines = Vec::new();
    for line in input.split("\n").into_iter() {
        lines.push(String::from(line));
    }
    lines
}
