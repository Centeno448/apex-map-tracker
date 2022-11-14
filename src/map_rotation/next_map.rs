use super::map_rotation_code::MapRotationCode;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct NextMap {
    pub code: MapRotationCode,
    #[serde(rename = "DurationInMinutes")]
    pub duration_in_minutes: u8,
}
