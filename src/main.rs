extern crate log;
extern crate simplelog;

use apex_map_tracker::configuration::Settings;
use apex_map_tracker::startup::{build_serenity_client, ShardManagerContainer};
use apex_map_tracker::telemetry::init_logger;
use dotenv;
use lazy_static::lazy_static;
use tracing::error;

use std::sync::Arc;
use tokio::sync::RwLock;

lazy_static! {
    pub static ref APP_SETTINGS: Arc<RwLock<Settings>> = Arc::new(RwLock::new(Settings::default()));
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().expect("Failed to load environment variables");

    init_logger();

    let mut client = build_serenity_client()
        .await
        .expect("Failed to create serenity client.");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(e) = client.start().await {
        error!("Failed to start client with error: {:?}", e);
    }

    Ok(())
}
