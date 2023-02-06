use crate::configuration::Settings;
use crate::map_rotation::{current_map, is_map_available};
use crate::map_rotation::{MapRotation, MapRotationCode};

use reqwest::Error;

pub async fn time_until(map: MapRotationCode, app_settings: &Settings) -> Result<String, Error> {
    let map_rotation = map_rotation_request(&app_settings.application.api_key).await?;

    let season_map_rotation = [
        MapRotationCode::BrokenMoonRotation,
        MapRotationCode::OlympusRotation,
        MapRotationCode::WorldsEdgeRotation,
    ];

    Ok(is_map_available(map_rotation, &map, &season_map_rotation))
}

pub async fn map(app_settings: &Settings) -> Result<String, Error> {
    let map_rotation = map_rotation_request(&app_settings.application.api_key).await?;

    Ok(current_map(map_rotation))
}

async fn map_rotation_request(api_token: &str) -> Result<MapRotation, Error> {
    let client = reqwest::Client::new();
    let request_url = format!("https://api.mozambiquehe.re/maprotation?auth={}", api_token);

    Ok(client
        .get(request_url)
        .send()
        .await?
        .json::<MapRotation>()
        .await?)
}
