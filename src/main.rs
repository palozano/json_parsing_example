/// Import the implementation for the data
use serde::{Deserialize, Serialize};

const URL_REDDIT: &str = "https://tradestie.com/api/v1/apps/reddit";

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RedditStock {
    no_of_comments: u32,
    sentiment: String,
    sentiment_score: f32,
    ticker: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CoinCapInfo {
    info: String,
    rank: u32,
    symbol: String,
    name: String,
    supply: f64,
    max_supply: Option<f64>,
    market_cap_usd: f64,
    volume_usd_24h: f64,
    price_usd: f64,
    change_percent_24h: f64,
    vwap_24h: f64,
    explorer: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let stocks = reqwest::Client::new()
        .get(URL_REDDIT)
        .send() // send the request
        .await? // await the response
        .json::<Vec<RedditStock>>() // deserealize the JSON response as a type Vec<Todo>
        .await?;

    let coins = reqwest::Client::new()
        .get(URL_COINCAP)
        .send() // send the request
        .await? // await the response
        .json::<Vec<CoinCapInfo>>() // deserealize the JSON response as a type Vec<Todo>
        .json() // deserealize the JSON response as a type Vec<Todo>
        .await?;
    println!("{:#?}", coins);

    let best_tickets = stocks
        .into_iter()
        .filter(|s| s.sentiment_score > 0.5)
        .collect::<Vec<RedditStock>>();

    if best_tickets.len() > 10 {
        println!("10 best Reddit tickers -> {:#?}", &best_tickets[0..10]);
    } else {
        println!(
            "{} best Reddit tickers -> {:#?}",
            best_tickets.len(),
            &best_tickets
        );
    }

    Ok(())
}
