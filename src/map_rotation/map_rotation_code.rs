use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum MapRotationCode {
    #[serde(rename = "worlds_edge_rotation")]
    WorldsEdgeRotation,
    #[serde(rename = "storm_point_rotation")]
    StormPointRotation,
    #[serde(rename = "olympus_rotation")]
    OlympusRotation,
}
