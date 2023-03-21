use sqlx::{query, MySqlPool};

use crate::commands::result::CommandResult;
use crate::configuration::Settings;
use crate::map_rotation::{current_br_map, current_ltm, is_br_map_available};
use crate::map_rotation::{MapRotationCode, Rotations};

pub async fn time_until(
    map: MapRotationCode,
    app_settings: &Settings,
    db_pool: &MySqlPool,
) -> CommandResult<String> {
    let rotations = map_rotation_request(
        &app_settings.application.api_base_url,
        &app_settings.application.api_key,
    )
    .await?;

    let rows = query!("SELECT code FROM map_rotations",)
        .fetch_all(db_pool)
        .await?;

    let mut season_map_rotation: Vec<MapRotationCode> = Vec::new();

    for row in rows {
        season_map_rotation.push(row.code.as_str().into());
    }

    Ok(is_br_map_available(
        rotations.battle_royale,
        &map,
        &season_map_rotation,
    ))
}

pub async fn map(app_settings: &Settings) -> CommandResult<String> {
    let rotations = map_rotation_request(
        &app_settings.application.api_base_url,
        &app_settings.application.api_key,
    )
    .await?;

    Ok(current_br_map(rotations.battle_royale))
}

pub async fn ltm(app_settings: &Settings) -> CommandResult<String> {
    let rotations = map_rotation_request(
        &app_settings.application.api_base_url,
        &app_settings.application.api_key,
    )
    .await?;

    Ok(current_ltm(rotations.ltm))
}

async fn map_rotation_request(base_url: &str, api_token: &str) -> CommandResult<Rotations> {
    let client = reqwest::Client::new();
    let request_url = format!("{}/maprotation?auth={}&version=2", base_url, api_token);

    Ok(client
        .get(request_url)
        .send()
        .await?
        .json::<Rotations>()
        .await?)
}
