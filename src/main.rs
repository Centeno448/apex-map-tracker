use apex_map_tracker::configuration::get_configuration;
use apex_map_tracker::startup::run;
use apex_map_tracker::telemetry::init_logger;
use dotenv;
use sqlx::mysql::MySqlPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().expect("Failed to load .env file");

    let configuration = get_configuration().expect("Failed to load configuration.");

    let connection_pool =
        MySqlPoolOptions::new().connect_lazy_with(configuration.database.with_db());

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    init_logger(&configuration);

    run(configuration, connection_pool).await?.start().await?;

    Ok(())
}
