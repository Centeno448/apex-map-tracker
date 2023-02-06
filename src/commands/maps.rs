use crate::map_rotation::{current_map, is_map_available};
use crate::map_rotation::{MapRotation, MapRotationCode};

use reqwest::Error;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub async fn olympus(ctx: &Context, msg: &Message) -> CommandResult {
    let map_rotation = map_rotation_request().await?;
    let season_map_rotation = [
        MapRotationCode::WorldsEdgeRotation,
        MapRotationCode::WorldsEdgeRotation,
        MapRotationCode::WorldsEdgeRotation,
    ];

    let response = is_map_available(
        map_rotation,
        &MapRotationCode::OlympusRotation,
        &season_map_rotation,
    );

    msg.channel_id.say(&ctx.http, response).await?;

    Ok(())
}

#[command]
pub async fn fin(ctx: &Context, msg: &Message) -> CommandResult {
    let map_rotation = map_rotation_request().await?;
    let season_map_rotation = [
        MapRotationCode::WorldsEdgeRotation,
        MapRotationCode::WorldsEdgeRotation,
        MapRotationCode::WorldsEdgeRotation,
    ];

    let response = is_map_available(
        map_rotation,
        &MapRotationCode::WorldsEdgeRotation,
        &season_map_rotation,
    );

    msg.channel_id.say(&ctx.http, response).await?;

    Ok(())
}

#[command]
pub async fn kings(ctx: &Context, msg: &Message) -> CommandResult {
    let map_rotation = map_rotation_request().await?;
    let season_map_rotation = [
        MapRotationCode::WorldsEdgeRotation,
        MapRotationCode::WorldsEdgeRotation,
        MapRotationCode::WorldsEdgeRotation,
    ];

    let response = is_map_available(
        map_rotation,
        &MapRotationCode::KingsCanyonRotation,
        &season_map_rotation,
    );

    msg.channel_id.say(&ctx.http, response).await?;

    Ok(())
}

#[command]
pub async fn punto(ctx: &Context, msg: &Message) -> CommandResult {
    let map_rotation = map_rotation_request().await?;
    let season_map_rotation = [
        MapRotationCode::WorldsEdgeRotation,
        MapRotationCode::WorldsEdgeRotation,
        MapRotationCode::WorldsEdgeRotation,
    ];

    let response = is_map_available(
        map_rotation,
        &MapRotationCode::StormPointRotation,
        &season_map_rotation,
    );

    msg.channel_id.say(&ctx.http, response).await?;

    Ok(())
}

#[command]
pub async fn moon(ctx: &Context, msg: &Message) -> CommandResult {
    let map_rotation = map_rotation_request().await?;
    let season_map_rotation = [
        MapRotationCode::WorldsEdgeRotation,
        MapRotationCode::WorldsEdgeRotation,
        MapRotationCode::WorldsEdgeRotation,
    ];

    let response = is_map_available(
        map_rotation,
        &MapRotationCode::BrokenMoonRotation,
        &season_map_rotation,
    );

    msg.channel_id.say(&ctx.http, response).await?;

    Ok(())
}

#[command]
pub async fn map(ctx: &Context, msg: &Message) -> CommandResult {
    let map_rotation = map_rotation_request().await?;

    let response = current_map(map_rotation);

    msg.channel_id.say(&ctx.http, response).await?;

    Ok(())
}

async fn map_rotation_request() -> Result<MapRotation, Error> {
    let client = reqwest::Client::new();
    let request_url =
        "https://api.mozambiquehe.re/maprotation?auth=8d1fba7b17cf0d8604250b656b2eee70";

    Ok(client
        .get(request_url)
        .send()
        .await?
        .json::<MapRotation>()
        .await?)
}
