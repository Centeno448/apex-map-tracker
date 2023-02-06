use std::fmt::Display;

use reqwest::Error;

use crate::commands::maps::*;
use crate::configuration::Settings;
use crate::map_rotation::MapRotationCode;

pub enum Command {
    FinCommand,
    KingsCommand,
    MapCommand,
    MoonCommand,
    OlympusCommand,
    PuntoCommand,
}

impl Command {
    pub fn parse(str: &str) -> Option<Command> {
        match str {
            "olympus" => Some(Command::OlympusCommand),
            "moon" => Some(Command::MoonCommand),
            "fin" => Some(Command::FinCommand),
            "kings" => Some(Command::KingsCommand),
            "punto" => Some(Command::PuntoCommand),
            "map" => Some(Command::MapCommand),
            _ => None,
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OlympusCommand => write!(f, "{}", "olympus"),
            Self::MoonCommand => write!(f, "{}", "moon"),
            Self::FinCommand => write!(f, "{}", "fin"),
            Self::KingsCommand => write!(f, "{}", "kings"),
            Self::PuntoCommand => write!(f, "{}", "punto"),
            Self::MapCommand => write!(f, "{}", "map"),
        }
    }
}

pub async fn handle_command(command: &Command, app_settings: &Settings) -> Result<String, Error> {
    match command {
        Command::OlympusCommand => time_until(MapRotationCode::OlympusRotation, app_settings).await,
        Command::MoonCommand => time_until(MapRotationCode::BrokenMoonRotation, app_settings).await,
        Command::FinCommand => time_until(MapRotationCode::WorldsEdgeRotation, app_settings).await,
        Command::KingsCommand => {
            time_until(MapRotationCode::KingsCanyonRotation, app_settings).await
        }
        Command::PuntoCommand => {
            time_until(MapRotationCode::StormPointRotation, app_settings).await
        }
        Command::MapCommand => map(app_settings).await,
    }
}
