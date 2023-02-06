use crate::map_rotation::MapRotationCode;
use std::env;

#[derive(Debug, serde::Deserialize)]
pub struct Settings {
    pub api_base_url: String,
    pub api_key: String,
    pub discord_bot_key: String,
    pub map_rotation_url: String,
    pub season_map_rotation: [MapRotationCode; 3],
}

pub fn get_configuration() -> Settings {
    let api_base_url =
        env::var("AMC_BASE_API_URL").expect("Could not find AMC_BASE_API_URL in env variables");
    let api_key = env::var("AMC_API_KEY").expect("Could not find AMC_API_KEY in env variables");
    let discord_bot_key = env::var("AMC_DISCORD_BOT_TOKEN")
        .expect("Could not find AMC_DISCORD_BOT_TOKEN in env variables");

    let map_rotation_url = format!("{}/maprotation?auth={}", api_base_url, api_key);

    let season_map_rotation = [
        MapRotationCode::BrokenMoonRotation,
        MapRotationCode::OlympusRotation,
        MapRotationCode::WorldsEdgeRotation,
    ];

    Settings {
        api_base_url,
        api_key,
        discord_bot_key,
        map_rotation_url,
        season_map_rotation,
    }
}
