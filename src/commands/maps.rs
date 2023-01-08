use crate::map_rotation::{current_map, is_map_available};
use crate::map_rotation::{MapRotation, MapRotationCode};
use crate::APP_SETTINGS;

use reqwest::Error;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub async fn olympus(ctx: &Context, msg: &Message) -> CommandResult {
    let map_rotation = map_rotation_request().await?;
    let response = is_map_available(map_rotation, &MapRotationCode::OlympusRotation).await;

    msg.channel_id.say(&ctx.http, response).await?;

    Ok(())
}

#[command]
pub async fn fin(ctx: &Context, msg: &Message) -> CommandResult {
    let map_rotation = map_rotation_request().await?;
    let response = is_map_available(map_rotation, &MapRotationCode::WorldsEdgeRotation).await;

    msg.channel_id.say(&ctx.http, response).await?;

    Ok(())
}

#[command]
pub async fn kings(ctx: &Context, msg: &Message) -> CommandResult {
    let map_rotation = map_rotation_request().await?;
    let response = is_map_available(map_rotation, &MapRotationCode::KingsCanyonRotation).await;

    msg.channel_id.say(&ctx.http, response).await?;

    Ok(())
}

#[command]
pub async fn punto(ctx: &Context, msg: &Message) -> CommandResult {
    let map_rotation = map_rotation_request().await?;
    let response = is_map_available(map_rotation, &MapRotationCode::StormPointRotation).await;

    msg.channel_id.say(&ctx.http, response).await?;

    Ok(())
}

#[command]
pub async fn moon(ctx: &Context, msg: &Message) -> CommandResult {
    let map_rotation = map_rotation_request().await?;
    let response = is_map_available(map_rotation, &MapRotationCode::BrokenMoonRotation).await;

    msg.channel_id.say(&ctx.http, response).await?;

    Ok(())
}

#[command]
pub async fn map(ctx: &Context, msg: &Message) -> CommandResult {
    let map_rotation = map_rotation_request().await?;

    let response = current_map(map_rotation).await;

    msg.channel_id.say(&ctx.http, response).await?;

    Ok(())
}

async fn map_rotation_request() -> Result<MapRotation, Error> {
    let client = reqwest::Client::new();
    let request_url = &APP_SETTINGS.read().await.map_rotation_url;

    Ok(client
        .get(request_url)
        .send()
        .await?
        .json::<MapRotation>()
        .await?)
}
