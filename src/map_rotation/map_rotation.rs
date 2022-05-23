use super::current_map::CurrentMap;
use super::next_map::NextMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MapRotation {
    current: CurrentMap,
    next: NextMap,
}
