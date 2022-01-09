use reqwest::Error;
use helpers::input::*;

mod helpers {
  pub mod input;
}

mod day_01;


#[tokio::main]
async fn main() -> Result<(), Error>{
  // Load environment variables from .env file
  dotenv::dotenv().ok();
  
  let input = lines_to_vec(fetch_input(1).await?);
  let day_1_part_1_answer = day_01::part_1(&input);
  println!("Day 1, part 1: Number of increases is {:?}", day_1_part_1_answer);
  assert_eq!(day_1_part_1_answer, 1226);

  Ok(())
}