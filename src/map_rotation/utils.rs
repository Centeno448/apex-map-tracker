use super::{next_map::NextMap, MapRotationCode};

pub fn calculate_time_to_map_in_minutes(
    map: &MapRotationCode,
    cm_remaining: &u8,
    next_map: &NextMap,
) -> u8 {
    if next_map.code == *map {
        return *cm_remaining;
    }

    cm_remaining + next_map.duration_in_minutes
}
