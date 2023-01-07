use crate::map_rotation::is_map_available;
use crate::map_rotation::MapRotationCode;

use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub async fn olympus(ctx: &Context, msg: &Message) -> CommandResult {
    let response = is_map_available(&MapRotationCode::OlympusRotation).await?;

    msg.channel_id.say(&ctx.http, response).await?;

    Ok(())
}
