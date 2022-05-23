use super::map_rotation_code::MapRotationCode;
use serde::Deserialize;

#[derive(Deserialize, Debug)]

pub struct CurrentMap {
    #[serde(rename = "readableDate_start")]
    readable_date_start: String,
    #[serde(rename = "readableDate_end")]
    readable_date_end: String,
    map: String,
    code: MapRotationCode,
    #[serde(rename = "remainingSecs")]
    remaining_secs: u16,
    #[serde(rename = "remainingMins")]
    remaining_mins: u8,
    #[serde(rename = "remainingTimer")]
    remaining_timer: String,
}
