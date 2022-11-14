use crate::map_rotation::MapRotationCode;
use crate::APP_SETTINGS;

use crate::map_rotation::{calculate_time_to_map_in_minutes, MapRotation};

use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub async fn olympus(ctx: &Context, msg: &Message) -> CommandResult {
    let client = reqwest::Client::new();
    let request_url = &APP_SETTINGS.read().await.map_rotation_url;

    let resp = client
        .get(request_url)
        .send()
        .await?
        .json::<MapRotation>()
        .await?;

    let current_map = resp.current.code;

    if current_map == MapRotationCode::OlympusRotation {
        let time_left = &resp.current.remaining_timer;
        let response = format!("En efecto, está olympus. Tiempo restante: {time_left}");

        msg.channel_id.say(&ctx.http, response).await?;
    } else {
        let time_until = calculate_time_to_map_in_minutes(
            &MapRotationCode::OlympusRotation,
            &resp.current.remaining_mins,
            &resp.next,
        );

        let response =
            format!("Nel, actualmente está {current_map}. Olympus estára en {time_until} minutos.");

        msg.channel_id.say(&ctx.http, response).await?;
    }

    Ok(())
}
