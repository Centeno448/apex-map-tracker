use core::fmt;

use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub enum MapRotationCode {
    #[serde(rename = "worlds_edge_rotation")]
    WorldsEdgeRotation,
    #[serde(rename = "storm_point_rotation")]
    StormPointRotation,
    #[serde(rename = "olympus_rotation")]
    OlympusRotation,
    #[serde(rename = "broken_moon_rotation")]
    BrokenMoonRotation,
}

impl fmt::Display for MapRotationCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.

        match self {
            Self::WorldsEdgeRotation => write!(f, "{}", "Worlds Edge"),
            Self::StormPointRotation => write!(f, "{}", "Storm Point"),
            Self::OlympusRotation => write!(f, "{}", "Olympus"),
            Self::BrokenMoonRotation => write!(f, "{}", "Broken Moon"),
        }
    }
}
