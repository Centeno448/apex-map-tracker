extern crate log;
extern crate simplelog;

use apex_map_tracker::configuration::get_configuration;
use apex_map_tracker::startup::run;
use apex_map_tracker::telemetry::init_logger;
use dotenv;
use tracing::error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().expect("Failed to load environment variables");

    init_logger();

    let app_settings = get_configuration();

    if let Err(e) = run(app_settings).await?.start().await {
        error!("Failed to start client with error: {:?}", e);
    }

    Ok(())
}
