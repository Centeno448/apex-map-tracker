use serenity::async_trait;
use serenity::client::bridge::gateway::ShardManager;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::model::prelude::Message;
use serenity::prelude::*;
use std::sync::Arc;
use tracing::{error, info};

use crate::commands::{handle_command, Command};
use crate::configuration::Settings;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler {
    app_settings: Settings,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let prefix = "e?";
        if let Some(command_str) = msg.content.strip_prefix(prefix) {
            if let Some(command) = Command::parse(command_str) {
                match handle_command(&command, &self.app_settings).await {
                    Ok(response) => match msg.channel_id.say(&ctx, response).await {
                        Ok(_) => (),
                        Err(e) => {
                            let error_message =
                                format!("Failed to send response with error: {:?}", e);
                            error!(error_message);
                        }
                    },
                    Err(e) => {
                        let error_message = format!(
                            "Failed to execute command {prefix}{} with error: {:?}",
                            &command, e
                        );
                        error!(error_message);
                    }
                }
            }
        }
    }
}

pub async fn build_serenity_client(app_settings: Settings) -> Result<Client, SerenityError> {
    let token = app_settings.discord_bot_key.clone();

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let handler = Handler { app_settings };

    Client::builder(&token, intents)
        .event_handler(handler)
        .await
}
