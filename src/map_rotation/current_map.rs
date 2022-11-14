use super::map_rotation_code::MapRotationCode;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CurrentMap {
    #[serde(rename = "readableDate_start")]
    pub readable_date_start: String,
    #[serde(rename = "readableDate_end")]
    pub readable_date_end: String,
    pub map: String,
    pub code: MapRotationCode,
    #[serde(rename = "remainingSecs")]
    pub remaining_secs: u16,
    #[serde(rename = "remainingMins")]
    pub remaining_mins: u8,
    #[serde(rename = "remainingTimer")]
    pub remaining_timer: String,
}
