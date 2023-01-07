use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Color;

#[command]
pub async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Comandos")
                    .description(
                        "Estos son los comandos que el bot tiene implementado de a momento.",
                    )
                    .fields(vec![
                        (
                            "e?olympus",
                            "Calcula el tiempo hasta que Olympus esté en la rotación.",
                            false,
                        ),
                        (
                            "e?moon",
                            "Calcula el tiempo hasta que Broken Moon esté en la rotación.",
                            false,
                        ),
                        (
                            "e?fin",
                            "Calcula el tiempo hasta que Fin del Mundo esté en la rotación.",
                            false,
                        ),
                        (
                            "e?kings",
                            "Calcula el tiempo hasta que Kings Canyon esté en la rotación.",
                            false,
                        ),
                        (
                            "e?punto",
                            "Calcula el tiempo hasta que Punto Tormenta esté en la rotación.",
                            false,
                        ),
                        (
                            "e?map",
                            "Muestra el mapa actual y el tiempo restante en rotación.",
                            false,
                        ),
                        (
                            "e?help",
                            "Despliega un listado de los comandos disponibles.",
                            false,
                        ),
                    ])
                    .color(Color::from_rgb(116, 199, 8))
            })
        })
        .await?;

    Ok(())
}
