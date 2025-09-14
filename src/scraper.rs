use crate::models::PricesResponse;
use reqwest::Error;

pub async fn fetch_prices(coins: &[&str]) -> Result<PricesResponse, Error> {
    let ids = coins.join(",");
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd",
        ids
    );

    let resp = reqwest::get(&url).await?.json::<PricesResponse>().await?;
    Ok(resp)
}
