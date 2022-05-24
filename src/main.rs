#![allow(dead_code)]

use dotenv;
use map_rotation::MapRotation;
use std::env;

mod map_rotation;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let base_url = env::var("AMC_BASE_API_URL")?;
    let api_key = env::var("AMC_API_KEY")?;

    let request_url = format!("{}/maprotation?auth={}", base_url, api_key);

    let client = reqwest::Client::new();

    let resp = client
        .get(request_url)
        .send()
        .await?
        .json::<MapRotation>()
        .await?;
    println!("{:#?}", resp);

    Ok(())
}
