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
    let problem_num = get_problem_from_args()
        .or_else(get_problem_from_input)
        .or_else(get_problem_from_date)
        .ok_or("Unable to get problem number!")?;
    let cookie = fs::read_to_string("cookie").await?;

    let input_link: Url =
        format!("https://adventofcode.com/2022/day/{problem_num}/input").parse()?;
    let input_res = Client::new()
        .get(input_link)
        .header(COOKIE, &cookie)
        .send()
        .await?;

    let dir_path = format!("./src/day_{problem_num}");
    let dir_path = Path::new(&dir_path);
    fs::create_dir_all(dir_path).await?;
    fs::write(dir_path.join(Path::new("input")), input_res.text().await?).await?;

    let problem_name = get_problem_name(problem_num, &cookie)
        .await?
        .replace('-', "_");
    let problem_file = problem_name.clone() + ".rs";
    let path = dir_path.join(Path::new(&problem_file));
    let problem_exist = path.exists();
    if problem_exist {
        println!("File '{problem_file}' already exists")
    } else {
        let link: Url = format!("https://adventofcode.com/2022/day/{problem_num}").parse()?;
        let tmpl = CODE_TMPL
            .replace(PROBLEM_NAME_TMPL, &problem_name)
            .replace(PROBLEM_LINK_TMPL, link.as_ref());
        fs::write(&path, tmpl).await?;
    }

    let mut file = OpenOptions::new().append(true).open("Cargo.toml").await?;
    if !problem_exist {
        file.write_all(
            TOML_TMPL
                .replace(PROBLEM_NAME_TMPL, &problem_name)
                .replace(PROBLEM_NUM_TMPL, &problem_num.to_string())
                .as_bytes(),
        )
        .await?;
    }

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
    let (_, txt) = txt.split_once(":").expect("':' inside h2");
    let (txt, _) = txt.split_once("---").expect("'---' inside h2");
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
    //https://docs.rs/chrono/0.4.23/chrono/
    let time_zone = FixedOffset::east_opt(-5 * 3600).unwrap();
    let time = Utc::now().with_timezone(&time_zone);
    let time_zone = FixedOffset::east_opt(-5 * 3600).unwrap();

    let start = time_zone.with_ymd_and_hms(2022, 12, 1, 0, 0, 0).unwrap();
    let end = time_zone.with_ymd_and_hms(2022, 12, 26, 0, 0, 0).unwrap();

    if start <= time && time < end {
        let day = time.day();
        println!("Today day's challenge: {day}");
        Some(day as u8)
    } else {
        None
    }
}

const PROBLEM_NAME_TMPL: &str = "{PROBLEM_NAME}";
const PROBLEM_LINK_TMPL: &str = "{PROBLEM_LINK}";
const CODE_TMPL: &str = r##"
use advent_2022::get_input_str;
use itertools::Itertools;

// {PROBLEM_LINK}

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
    use super::*;

    const INPUT: &str = r#"
"#;

    fn get_input(input: &'static str) -> &'static str {
        input.strip_prefix('\n').unwrap().strip_suffix('\n').unwrap()
    }

    #[test]
    fn parse_test() {
        let parsed = parse(get_input(INPUT));
        dbg!(&parsed);
    }

    #[test]
    fn test_1() {
        let expected = todo!();
        let ans = {PROBLEM_NAME}_1(get_input(INPUT));
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_2() {
        let expected = todo!();
        let ans = {PROBLEM_NAME}_2(get_input(INPUT));
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
