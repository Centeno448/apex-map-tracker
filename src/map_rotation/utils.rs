use super::{next_map::NextMap, MapRotation, MapRotationCode};

use crate::APP_SETTINGS;

fn calculate_time_to_map_in_minutes(
    map: &MapRotationCode,
    cm_remaining: &u8,
    next_map: &NextMap,
) -> u8 {
    if next_map.code == *map {
        return *cm_remaining;
    }

    cm_remaining + next_map.duration_in_minutes
}

pub async fn is_map_available(rotation: MapRotation, map: &MapRotationCode) -> String {
    let season_map_rotation = &APP_SETTINGS.read().await.season_map_rotation;

    if !season_map_rotation.contains(map) {
        return format!("{map} no está en la rotación de esta temporada :C");
    }

    let current_map = rotation.current.code;

    if current_map == *map {
        let time_left = &rotation.current.remaining_timer;
        return format!("En efecto, está {map}. Tiempo restante: {time_left}");
    } else {
        let time_until = calculate_time_to_map_in_minutes(
            &map,
            &rotation.current.remaining_mins,
            &rotation.next,
        );

        return format!(
            "Nel, actualmente está {current_map}. {map} estára en {time_until} minutos."
        );
    }
}

pub async fn current_map(rotation: MapRotation) -> String {
    let current_map = rotation.current.code;
    let time_left = &rotation.current.remaining_timer;

    format!("El mapa actual es {current_map}. Tiempo restante: {time_left}")
}

#[cfg(test)]
mod tests {
    use super::{calculate_time_to_map_in_minutes, current_map};

    mod calculate_time_to_map_in_minutes {
        use super::calculate_time_to_map_in_minutes;
        use crate::map_rotation::{next_map::NextMap, MapRotationCode};

        #[test]
        fn when_search_is_next_map_returns_current_map_remaining_() {
            let next_map = NextMap {
                code: MapRotationCode::BrokenMoonRotation,
                duration_in_minutes: 100,
            };
            let actual = calculate_time_to_map_in_minutes(
                &MapRotationCode::BrokenMoonRotation,
                &10,
                &next_map,
            );
            let expected = 10;

            assert_eq!(actual, expected);
        }

        #[test]
        fn when_search_is_not_next_map_returns_sum_of_current_map_remaining_and_next_map_duration_()
        {
            let next_map = NextMap {
                code: MapRotationCode::BrokenMoonRotation,
                duration_in_minutes: 100,
            };
            let actual =
                calculate_time_to_map_in_minutes(&MapRotationCode::OlympusRotation, &10, &next_map);

            let expected = 110;

            assert_eq!(actual, expected);
        }
    }

    mod current_map {
        use crate::map_rotation::{
            current_map::CurrentMap, next_map::NextMap, MapRotation, MapRotationCode,
        };

        use super::current_map;

        #[actix_rt::test]
        async fn returns_the_correct_string() {
            let rotation = MapRotation {
                current: CurrentMap {
                    code: MapRotationCode::BrokenMoonRotation,
                    remaining_mins: 10,
                    remaining_timer: String::from("00:10:00"),
                },
                next: NextMap {
                    code: MapRotationCode::KingsCanyonRotation,
                    duration_in_minutes: 1,
                },
            };

            let expected = "El mapa actual es Broken Moon. Tiempo restante: 00:10:00";
            let actual = current_map(rotation).await;

            assert_eq!(expected, actual);
        }
    }
}
