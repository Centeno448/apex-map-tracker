use super::map_rotation_code::MapRotationCode;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CurrentMap {
    pub code: MapRotationCode,
    #[serde(rename = "remainingMins")]
    pub remaining_mins: u8,
    #[serde(rename = "remainingTimer")]
    pub remaining_timer: String,
}
