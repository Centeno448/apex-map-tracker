use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CurrentLTM {
    #[serde(rename = "eventName")]
    pub event_name: String,
    #[serde(rename = "remainingMins")]
    pub remaining_mins: u8,
    #[serde(rename = "remainingTimer")]
    pub remaining_timer: String,
}

#[derive(Deserialize, Debug)]
pub struct NextLTM {
    #[serde(rename = "eventName")]
    pub event_name: String,
    #[serde(rename = "DurationInMinutes")]
    pub duration_in_minutes: u8,
}
