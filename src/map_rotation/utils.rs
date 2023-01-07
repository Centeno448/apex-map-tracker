use super::{next_map::NextMap, MapRotation, MapRotationCode};
use reqwest::Error;

use crate::APP_SETTINGS;

fn calculate_time_to_map_in_minutes(
    map: &MapRotationCode,
    cm_remaining: &u8,
    next_map: &NextMap,
) -> u8 {
    if next_map.code == *map {
        return *cm_remaining;
    }

    cm_remaining + next_map.duration_in_minutes
}

pub async fn is_map_available(map: &MapRotationCode) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let request_url = &APP_SETTINGS.read().await.map_rotation_url;
    let season_map_rotation = &APP_SETTINGS.read().await.season_map_rotation;

    if !season_map_rotation.contains(map) {
        return Ok(format!("{map} no está en la rotación de esta temporada :C"));
    }

    let resp = client
        .get(request_url)
        .send()
        .await?
        .json::<MapRotation>()
        .await?;

    let current_map = resp.current.code;

    if current_map == *map {
        let time_left = &resp.current.remaining_timer;
        return Ok(format!(
            "En efecto, está {map}. Tiempo restante: {time_left}"
        ));
    } else {
        let time_until =
            calculate_time_to_map_in_minutes(&map, &resp.current.remaining_mins, &resp.next);

        return Ok(format!(
            "Nel, actualmente está {current_map}. {map} estára en {time_until} minutos."
        ));
    }
}
