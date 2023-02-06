extern crate log;
extern crate simplelog;

use apex_map_tracker::configuration::get_configuration;
use apex_map_tracker::startup::{build_serenity_client, ShardManagerContainer};
use apex_map_tracker::telemetry::init_logger;
use dotenv;
use tracing::error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().expect("Failed to load environment variables");

    init_logger();

    let app_settings = get_configuration();

    let mut client = build_serenity_client(app_settings)
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
