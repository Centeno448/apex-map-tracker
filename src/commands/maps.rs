use crate::map_rotation::MapRotationCode;
use crate::map_rotation::{current_map, is_map_available};

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

#[command]
pub async fn fin(ctx: &Context, msg: &Message) -> CommandResult {
    let response = is_map_available(&MapRotationCode::WorldsEdgeRotation).await?;

    msg.channel_id.say(&ctx.http, response).await?;

    Ok(())
}

#[command]
pub async fn kings(ctx: &Context, msg: &Message) -> CommandResult {
    let response = is_map_available(&MapRotationCode::KingsCanyonRotation).await?;

    msg.channel_id.say(&ctx.http, response).await?;

    Ok(())
}

#[command]
pub async fn punto(ctx: &Context, msg: &Message) -> CommandResult {
    let response = is_map_available(&MapRotationCode::StormPointRotation).await?;

    msg.channel_id.say(&ctx.http, response).await?;

    Ok(())
}

#[command]
pub async fn moon(ctx: &Context, msg: &Message) -> CommandResult {
    let response = is_map_available(&MapRotationCode::BrokenMoonRotation).await?;

    msg.channel_id.say(&ctx.http, response).await?;

    Ok(())
}

#[command]
pub async fn map(ctx: &Context, msg: &Message) -> CommandResult {
    let response = current_map().await?;

    msg.channel_id.say(&ctx.http, response).await?;

    Ok(())
}
