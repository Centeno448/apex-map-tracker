use super::battle_royale::{CurrentMap, NextMap};
use super::ltm::{CurrentLTM, NextLTM};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BRRotation {
    pub current: CurrentMap,
    pub next: NextMap,
}

#[derive(Debug, Deserialize)]
pub struct LTMRotation {
    pub current: CurrentLTM,
    pub next: NextLTM,
}

#[derive(Deserialize, Debug)]
pub struct Rotations {
    pub battle_royale: BRRotation,
    pub ltm: LTMRotation,
}
