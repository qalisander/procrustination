use reqwest::header::COOKIE;
use reqwest::Client;
use tokio::fs;

// TODO: #q create folder if not exist and add problem template and link
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let problem_number: u8 = std::env::args()
        .nth(1)
        .as_deref()
        .map(str::parse)
        .and_then(Result::ok)
        .expect("Must provide problem number to fetch input");
    
    // TODO: #q fetch for today if number is not indicated

    let cookie = fs::read_to_string(".cookie").await?;
    let url = format!("https://adventofcode.com/2022/day/{problem_number}/input");
    let res = Client::new().get(url).header(COOKIE, cookie).send().await?;
    
    // TODO: #q setup gpg. Add encrypted cookie
    fs::write(format!("./src/day_{problem_number}.txt"), res.text().await?).await?;

    // TODO: #q use scrapper to get name of a problem and description https://github.com/causal-agent/scraper
    Ok(())
}
