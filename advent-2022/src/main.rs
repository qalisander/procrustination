use chrono::prelude::*;
use reqwest::header::COOKIE;
use reqwest::Client;
use tokio::fs;

// TODO: #q use scrapper to get name of a problem and description https://github.com/causal-agent/scraper
// TODO: #q create folder if not exist and add problem template and link
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let problem_num = get_problem_from_args()
        .or_else(get_problem_from_input)
        .or_else(get_problem_from_date)
        .ok_or("Unable to get problem number")?;

    let cookie = fs::read_to_string("cookie").await?;
    let url = format!("https://adventofcode.com/2022/day/{problem_num}/input");
    let res = Client::new().get(url).header(COOKIE, cookie).send().await?;

    fs::write(format!("./src/day_{problem_num}.txt"), res.text().await?).await?;

    Ok(())
}

fn get_problem_from_args() -> Option<u8> {
    std::env::args().nth(1).as_deref()?.parse().ok()
}

fn get_problem_from_input() -> Option<u8> {
    println!("Enter a day's challenge to download: ");
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.parse().ok()
}

fn get_problem_from_date() -> Option<u8> {
    let time_zone = FixedOffset::east_opt(-5).unwrap();
    let time = Utc::now().with_timezone(&time_zone);
//    let start = Utc
//        .with_ymd_and_hms(2022, 12, 1, 0, 0, 0)
//        .unwrap()
//        .with_timezone(&time_zone);
//    let end = Utc.with_ymd_and_hms(2022, 12, 1, 0, 0, 0).unwrap();
    let day = time.day();
    println!("Today day's challenge {day}");
    Some(day as u8)
}
