use super::battle_royale::{CurrentMap, NextMap};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MapRotation {
    pub current: CurrentMap,
    pub next: NextMap,
}
