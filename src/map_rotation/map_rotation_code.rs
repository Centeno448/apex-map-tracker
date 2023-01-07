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
    #[serde(rename = "kings_canyon_rotation")]
    KingsCanyonRotation,
}

impl fmt::Display for MapRotationCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.

        match self {
            Self::WorldsEdgeRotation => write!(f, "{}", "Fin del Mundo"),
            Self::StormPointRotation => write!(f, "{}", "Punto Tormenta"),
            Self::OlympusRotation => write!(f, "{}", "Olympus"),
            Self::BrokenMoonRotation => write!(f, "{}", "Broken Moon"),
            Self::KingsCanyonRotation => write!(f, "{}", "Kings Canyon"),
        }
    }
}
