use chrono::prelude::*;
use reqwest::header::COOKIE;
use reqwest::{Client, Url};
use scraper::{Html, Selector};
use std::path::Path;
use tokio::fs;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: create struct. Use clap
    let problem_num = get_problem_from_args()
        .or_else(get_problem_from_input)
        .or_else(get_problem_from_date)
        .ok_or("Unable to get problem number!")?;
    let cookie = fs::read_to_string("cookie").await?;

    let input: Url = format!("https://adventofcode.com/2022/day/{problem_num}/input").parse()?;
    let input_res = Client::new()
        .get(input)
        .header(COOKIE, &cookie)
        .send()
        .await?;

    let dir_path = format!("./src/day_{problem_num}");
    let dir_path = Path::new(&dir_path);
    fs::create_dir_all(dir_path).await?;
    fs::write(dir_path.join(Path::new("input")), input_res.text().await?).await?;

    let problem_name = get_problem_name(problem_num, &cookie).await?;
    let problem_file = problem_name.clone() + ".rs";
    let path = dir_path.join(Path::new(&problem_file));
    if path.exists() {
        println!("File '{problem_file}' already exists")
    } else {
        fs::write(path, CODE_TMPL.replace(PROBLEM_NAME_TMPL, &problem_name)).await?;
    }

    let mut file = OpenOptions::new().append(true).open("Cargo.toml").await?;
    file.write_all(
        TOML_TMPL
            .replace(PROBLEM_NAME_TMPL, &problem_name)
            .replace(PROBLEM_NUM_TMPL, &problem_num.to_string())
            .as_bytes(),
    )
    .await?;

    Ok(())
}

async fn get_problem_name(
    problem_num: u8,
    cookie: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let html: Url = format!("https://adventofcode.com/2022/day/{problem_num}").parse()?;
    let html_res = Client::new()
        .get(html)
        .header(COOKIE, cookie)
        .send()
        .await?;
    let document = html_res.text().await?;
    let html = Html::parse_document(&document);

    let selector = Selector::parse("h2").unwrap();
    let txt = html.select(&selector).next().unwrap().html();
    let (_, txt) = txt.split_once(":").expect("String ':' is a part of h2 tag");
    let (txt, _) = txt
        .split_once("---")
        .expect("String '---' is a part of h2 tag");
    Ok(txt.trim().to_lowercase().replace(" ", "_"))
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

const PROBLEM_NAME_TMPL: &str = "{PROBLEM_NAME}";
const CODE_TMPL: &str = r##"
use advent_2022_rs::get_input_str;
use itertools::Itertools;

type Ans1 = todo!();
type Ans2 = todo!();

pub fn {PROBLEM_NAME}_1(input: &str) -> Ans1 {
    let parsed = parse(input);
    todo!("1")
}

pub fn {PROBLEM_NAME}_2(input: &str) -> Ans2 {
    let parsed = parse(input);
    todo!("2")
}

#[derive(Debug)]
struct Parsed;

fn parse(str: &str) -> Parsed {
    todo!("Parse")
}

fn main() {
    let str = get_input_str(file!());
    let ans = {PROBLEM_NAME}_1(&str);
    println!("Part 1: {ans}");
    let ans = {PROBLEM_NAME}_2(&str);
    println!("Part 2: {ans}");
}

#[cfg(test)]
mod tests {
    use crate::{parse, {PROBLEM_NAME}_1, {PROBLEM_NAME}_2};

    const INPUT: &str = r#""#;

    #[test]
    fn parse_test() {
        let parsed = parse(INPUT);
        dbg!(&parsed);
    }

    #[test]
    fn test_1() {
        let expected = todo!();
        let ans = {PROBLEM_NAME}_1(INPUT);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_2() {
        let expected = todo!();
        let ans = {PROBLEM_NAME}_2(INPUT);
        assert_eq!(ans, expected);
    }
}
"##;

const PROBLEM_NUM_TMPL: &str = "{PROBLEM_NUM}";
const TOML_TMPL: &str = r#"
[[bin]]
name = "day_{PROBLEM_NUM}"
path = "src/day_{PROBLEM_NUM}/{PROBLEM_NAME}.rs"
"#;
