extern crate log;
extern crate simplelog;

use config::AppConfig;
use dotenv;
use lazy_static::lazy_static;

use commands::OLYMPUS_COMMAND;
use serenity::async_trait;
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use simplelog::{Config, LevelFilter, WriteLogger};
use tracing::{error, info};

use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::sync::Arc;
use tokio::sync::RwLock;

mod commands;
mod config;
mod map_rotation;

lazy_static! {
    pub static ref APP_SETTINGS: Arc<RwLock<AppConfig>> =
        Arc::new(RwLock::new(AppConfig::default()));
}

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[group]
#[commands(olympus)]
struct General;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().expect("Failed to load environment variables");

    let base_path = env::var("AMC_BASE_LOG_PATH").unwrap();

    WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create(format!("{}/apex-map-tracker.log", base_path)).unwrap(),
    )
    .expect("Failed to initialize logger");

    let token = &APP_SETTINGS.read().await.discord_bot_key;

    let http = Http::new(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix("e?"))
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

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

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }

    Ok(())
}
