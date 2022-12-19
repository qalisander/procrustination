use std::ops::Deref;
use std::path::Path;
use chrono::prelude::*;
use reqwest::header::COOKIE;
use reqwest::{Client, Url};
use scraper::{Html, Selector};
use tokio::fs;
use tokio::io::AsyncWriteExt;

// TODO: #q use scrapper to get name of a problem and description https://github.com/causal-agent/scraper
// TODO: #q create folder if not exist and add problem template and link
// TODO: #q try to use clap here
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let problem_num = get_problem_from_args()
        .or_else(get_problem_from_input)
        .or_else(get_problem_from_date)
        .ok_or("Unable to get problem number!")?;

    let cookie = fs::read_to_string("cookie").await?;

    let input: Url = format!("https://adventofcode.com/2022/day/{problem_num}/input").parse()?;
    let input_res = Client::new().get(input).header(COOKIE, &cookie).send().await?;

    let path = format!("./src/day_{problem_num}/input.txt");
    let path = Path::new(&path);
    fs::create_dir_all(path.parent().unwrap()).await?;
    fs::write(path, input_res.text().await?).await?;

    let html: Url = format!("https://adventofcode.com/2022/day/{problem_num}").parse()?;
    let html_res = Client::new().get(html).header(COOKIE, &cookie).send().await?;
    let document = html_res.text().await?;
    let html = Html::parse_document(&document);

    let selector = Selector::parse("h2").unwrap();
    let txt = html.select(&selector).next().unwrap().html();
    dbg!(&txt);
    let (_, txt) = txt.split_once(":").expect("String ':' is a part of h2 tag");
    dbg!(&txt);
    let (txt, _) = txt.split_once("---").expect("String '---' is a part of h2 tag");
    let file_name = txt.trim().to_lowercase();

    Ok(())
}

fn get_problem_from_args() -> Option<u8> {
    std::env::args().nth(1).as_deref()?.parse().ok()
}

fn get_problem_from_input() -> Option<u8> {
    println!("Enter a day's challenge to download: ");
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim_end().parse().ok()
}

fn get_problem_from_date() -> Option<u8> {
    //    https://docs.rs/chrono/0.4.23/chrono/
    let time_zone = FixedOffset::east_opt(-5).unwrap();
    let time = Utc::now().with_timezone(&time_zone);
    let day = time.day();
    println!("Today day's challenge {day}");
    Some(day as u8)
}
