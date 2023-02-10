use simplelog::{Config, LevelFilter, SimpleLogger, WriteLogger};
use std::fs::File;

use crate::configuration::{Environment, Settings};

pub fn init_logger(configuration: &Settings) {
    let environment = &configuration.environment;

    match environment {
        Environment::Local => init_stdout_logger(),
        Environment::Production => init_file_logger(configuration),
    };
}

fn init_stdout_logger() {
    SimpleLogger::init(LevelFilter::Info, Config::default())
        .expect("Failed to initialize stdout logger");
}

fn init_file_logger(configuration: &Settings) {
    let base_path = &configuration.application.base_log_path;

    WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create(format!("{}/apex-map-tracker.log", base_path)).unwrap(),
    )
    .expect("Failed to initialize file logger");
}
