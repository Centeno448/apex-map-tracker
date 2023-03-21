use super::battle_royale::{CurrentMap, NextMap};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BRRotation {
    pub current: CurrentMap,
    pub next: NextMap,
}

#[derive(Deserialize, Debug)]
pub struct Rotations {
    pub battle_royale: BRRotation,
}
