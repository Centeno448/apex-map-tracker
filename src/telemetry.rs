use simplelog::{Config, LevelFilter, WriteLogger};
use std::fs::File;

use crate::configuration::Settings;

pub fn init_logger(configuration: &Settings) {
    let base_path = configuration.application.base_log_path.clone();

    WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create(format!("{}/apex-map-tracker.log", base_path)).unwrap(),
    )
    .expect("Failed to initialize logger");
}
