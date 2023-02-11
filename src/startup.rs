use serenity::async_trait;
use serenity::client::bridge::gateway::ShardManager;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::model::prelude::Message;
use serenity::prelude::*;
use sqlx::MySqlPool;
use std::sync::Arc;
use tracing::{info_span, Instrument};

use crate::commands::{handle_command, Command};
use crate::configuration::Settings;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

#[derive(Debug)]
struct Handler {
    app_settings: Settings,
    db_pool: MySqlPool,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        tracing::info!(
            "Connected as {} & ready to handle commands.",
            ready.user.name
        );
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        tracing::info!("Resumed.");
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let prefix = "e?";

        let command_str = match msg.content.strip_prefix(prefix) {
            Some(command_str) => command_str,
            None => {
                return ();
            }
        };

        let command = match Command::parse(command_str) {
            Some(command) => command,
            None => {
                return ();
            }
        };

        let span = info_span!("Handling command", command = msg.content);

        // Instrument the span
        async move {
            let response = match handle_command(&command, &self.app_settings, &self.db_pool).await {
                Ok(response) => response,
                Err(e) => {
                    tracing::error!("Failed to process command with error: {:?}", e);
                    return ();
                }
            };

            match msg.channel_id.say(&ctx, response).await {
                Ok(_) => (),
                Err(e) => {
                    tracing::error!("Failed to send response with error: {:?}", e);
                }
            }
        }
        .instrument(span)
        .await;
    }
}

pub async fn run(app_settings: Settings, db_pool: MySqlPool) -> Result<Client, SerenityError> {
    let token = app_settings.application.discord_bot_key.clone();

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let handler = Handler {
        app_settings,
        db_pool,
    };

    let client = Client::builder(&token, intents)
        .event_handler(handler)
        .await?;

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

    Ok(client)
}
