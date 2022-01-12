use helpers::input::*;
use reqwest::Error;

mod helpers {
    pub mod input;
}

mod day_01;
mod day_02;
mod day_03;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Day 1
    // let input = lines_to_vec(fetch_input(1).await?);
    // let day_1_part_1_answer = day_01::part_1(&input);
    // let day_1_part_2_answer = day_01::part_2(&input);
    // assert_eq!(day_1_part_1_answer, 1226);
    // assert_eq!(day_1_part_2_answer, 1252);

    // Day 2
    // let input = fetch_input(2).await?;
    // let instructions = day_02::get_instructions(&input);
    // let day_2_part_1_answer = day_02::part_1(&instructions);
    // let day_2_part_2_answer = day_02::part_2(&instructions);
    // assert_eq!(day_2_part_1_answer, 1815044);
    // assert_eq!(day_2_part_2_answer, 1739283308);

    // Day 3
    let input = lines_to_vec(fetch_input(3).await?);
    let day_3_part_1_answer = day_03::part_1(&input);
    assert_eq!(day_3_part_1_answer, 3242606);

    Ok(())
}
