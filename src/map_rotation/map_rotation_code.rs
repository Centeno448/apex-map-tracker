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
    #[serde(rename = "edistrict_rotation")]
    EDistrictRotation,
}

impl From<&str> for MapRotationCode {
    fn from(value: &str) -> Self {
        match value {
            "WorldsEdgeRotation" => MapRotationCode::WorldsEdgeRotation,
            "StormPointRotation" => MapRotationCode::StormPointRotation,
            "OlympusRotation" => MapRotationCode::OlympusRotation,
            "BrokenMoonRotation" => MapRotationCode::BrokenMoonRotation,
            "KingsCanyonRotation" => MapRotationCode::KingsCanyonRotation,
            "EDistrictRotation" => MapRotationCode::EDistrictRotation,
            _ => MapRotationCode::KingsCanyonRotation,
        }
    }
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
            Self::EDistrictRotation => write!(f, "{}", "E-District"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LTMCode {
    GunRun,
    TDM,
    Control,
    Unknown,
}

impl From<&str> for LTMCode {
    fn from(value: &str) -> Self {
        match value {
            "Gun Run" => LTMCode::GunRun,
            "TDM" => LTMCode::TDM,
            "Control" => LTMCode::Control,
            _ => LTMCode::Unknown,
        }
    }
}

impl fmt::Display for LTMCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.

        match self {
            Self::GunRun => write!(f, "{}", "Gun Run"),
            Self::TDM => write!(f, "{}", "Team Deathmatch"),
            Self::Control => write!(f, "{}", "Control"),
            Self::Unknown => write!(f, "{}", "Unknown"),
        }
    }
}

#[cfg(test)]
mod tests {

    mod map_rotation_code {
        use crate::map_rotation::MapRotationCode;

        #[test]
        fn displays_correctly() {
            assert_eq!(
                format!("{}", MapRotationCode::WorldsEdgeRotation),
                "Fin del Mundo"
            );
            assert_eq!(
                format!("{}", MapRotationCode::StormPointRotation),
                "Punto Tormenta"
            );
            assert_eq!(format!("{}", MapRotationCode::OlympusRotation), "Olympus");
            assert_eq!(
                format!("{}", MapRotationCode::BrokenMoonRotation),
                "Broken Moon"
            );
            assert_eq!(
                format!("{}", MapRotationCode::KingsCanyonRotation),
                "Kings Canyon"
            );
            assert_eq!(
                format!("{}", MapRotationCode::EDistrictRotation),
                "E-District"
            );
        }
    }

    mod ltm_code {
        use crate::map_rotation::LTMCode;

        #[test]
        fn displays_correctly() {
            assert_eq!(format!("{}", LTMCode::GunRun), "Gun Run");
            assert_eq!(format!("{}", LTMCode::TDM), "Team Deathmatch");
            assert_eq!(format!("{}", LTMCode::Control), "Control");
            assert_eq!(format!("{}", LTMCode::Unknown), "Unknown");
        }
    }
}
