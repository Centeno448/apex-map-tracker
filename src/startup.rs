use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use std::collections::HashSet;
use std::sync::Arc;

use crate::commands::{
    FIN_COMMAND, HELP_COMMAND, KINGS_COMMAND, MAP_COMMAND, MOON_COMMAND, OLYMPUS_COMMAND,
    PUNTO_COMMAND,
};
use crate::configuration::Settings;
use serenity::async_trait;
use serenity::framework::standard::macros::group;

use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use tracing::info;

use serenity::prelude::*;

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
}

#[group]
#[commands(olympus, moon, fin, kings, punto, map, help)]
struct General;

pub async fn build_serenity_client() -> Result<Client, SerenityError> {
    let token = "OTc4NDQ0ODUyOTM1MTM5Mzg4.G2BWMb.RKbPlmCWsrNrQhNBeip7r_DPYVjpcR-pqKsL-Y";

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

    let handler = Handler {
        app_settings: Settings::default(),
    };

    Client::builder(&token, intents)
        .framework(framework)
        .event_handler(handler)
        .await
}
