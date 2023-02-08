use core::fmt;
use sqlx::{query, MySqlPool};

use crate::configuration::Settings;
use crate::map_rotation::{current_map, is_map_available};
use crate::map_rotation::{MapRotation, MapRotationCode};

#[derive(Debug, Clone)]
pub struct CommandError(String);

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Command failed with error: {}", &self.0)
    }
}

impl From<reqwest::Error> for CommandError {
    fn from(value: reqwest::Error) -> Self {
        Self(value.to_string())
    }
}

impl From<sqlx::Error> for CommandError {
    fn from(value: sqlx::Error) -> Self {
        Self(value.to_string())
    }
}

type CommandResult<T> = std::result::Result<T, CommandError>;

pub async fn time_until(
    map: MapRotationCode,
    app_settings: &Settings,
    db_pool: &MySqlPool,
) -> CommandResult<String> {
    let map_rotation = map_rotation_request(&app_settings.application.api_key).await?;

    let rows = query!("SELECT code FROM map_rotations",)
        .fetch_all(db_pool)
        .await?;

    let mut season_map_rotation: Vec<MapRotationCode> = Vec::new();

    for row in rows {
        season_map_rotation.push(row.code.as_str().into());
    }

    Ok(is_map_available(map_rotation, &map, &season_map_rotation))
}

pub async fn map(app_settings: &Settings) -> CommandResult<String> {
    let map_rotation = map_rotation_request(&app_settings.application.api_key).await?;

    Ok(current_map(map_rotation))
}

async fn map_rotation_request(api_token: &str) -> CommandResult<MapRotation> {
    let client = reqwest::Client::new();
    let request_url = format!("https://api.mozambiquehe.re/maprotation?auth={}", api_token);

    Ok(client
        .get(request_url)
        .send()
        .await?
        .json::<MapRotation>()
        .await?)
}
