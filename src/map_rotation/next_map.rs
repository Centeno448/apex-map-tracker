use super::map_rotation_code::MapRotationCode;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct NextMap {
    #[serde(rename = "readableDate_start")]
    readable_date_start: String,
    #[serde(rename = "readableDate_end")]
    readable_date_end: String,
    map: String,
    pub code: MapRotationCode,
    #[serde(rename = "DurationInSecs")]
    duration_in_secs: u16,
    #[serde(rename = "DurationInMinutes")]
    pub duration_in_minutes: u8,
}
