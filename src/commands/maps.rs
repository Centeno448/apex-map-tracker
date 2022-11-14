use crate::map_rotation::MapRotationCode;
use crate::APP_SETTINGS;

use crate::map_rotation::MapRotation;

use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub async fn olympus(ctx: &Context, msg: &Message) -> CommandResult {
    let client = reqwest::Client::new();
    let request_url = &APP_SETTINGS.map_rotation_url;

    let resp = client
        .get(request_url)
        .send()
        .await?
        .json::<MapRotation>()
        .await?;

    let current_map = resp.current.code;

    if current_map == MapRotationCode::OlympusRotation {
        msg.channel_id
            .say(&ctx.http, "En efecto, esta olympus por ")
            .await?;
    } else {
        msg.channel_id.say(&ctx.http, "No esta olympus.").await?;
    }

    Ok(())
}
