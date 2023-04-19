/// Import the implementation for the data
use serde::{Deserialize, Serialize};

const URL_REDDIT: &str = "https://tradestie.com/api/v1/apps/reddit";
const URL_COINGECKO: &str = "https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&order=market_cap_desc&per_page=100&page=1&sparkline=false";

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RedditStock {
    no_of_comments: u32,
    sentiment: String,
    sentiment_score: f32,
    ticker: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Roi {
    times: f64,
    currency: String,
    percentage: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CoinGeckoInfo {
    id: String,
    symbol: String,
    name: String,
    image: String,
    current_price: f64,
    market_cap: f64,
    market_cap_rank: u32,
    fully_diluted_valuation: Option<f64>,
    total_volume: f64,
    high_24h: f64,
    low_24h: f64,
    price_change_24h: f64,
    price_change_percentage_24h: f64,
    market_cap_change_24h: f64,
    market_cap_change_percentage_24h: f64,
    circulating_supply: f64,
    total_supply: Option<f64>,
    max_supply: Option<f64>,
    ath: f64,
    ath_change_percentage: f64,
    ath_date: String,
    atl: f64,
    atl_change_percentage: f64,
    atl_date: String,
    roi: Option<Roi>,
    last_updated: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let stocks = reqwest::Client::new()
        .get(URL_REDDIT)
        .send() // send the request
        .await? // await the response
        .json::<Vec<RedditStock>>() // deserealize the JSON response as a type Vec<Todo>
        .await?;

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

    let coins = reqwest::Client::new()
        .get(URL_COINGECKO)
        .send() // send the request
        .await? // await the response
        .json::<Vec<CoinGeckoInfo>>() // deserealize the JSON response as a type Vec<Todo>
        .await?;
    println!("CoinGecko -> {:#?}", coins);

    Ok(())
}
