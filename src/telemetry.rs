use simplelog::{ConfigBuilder, LevelFilter, SimpleLogger, WriteLogger};
use std::fs::File;

use crate::configuration::{Environment, Settings};

pub fn init_logger(app_config: &Settings) {
    let environment = &app_config.environment;

    let log_config = ConfigBuilder::new()
        .add_filter_allow("apex_map_tracker".into())
        .build();

    match environment {
        Environment::Local => init_stdout_logger(log_config),
        Environment::Production => init_file_logger(log_config, app_config),
    };
}

fn init_stdout_logger(log_config: simplelog::Config) {
    SimpleLogger::init(LevelFilter::Info, log_config).expect("Failed to initialize stdout logger");
}

fn init_file_logger(log_config: simplelog::Config, app_config: &Settings) {
    let base_path = &app_config.application.base_log_path;

    WriteLogger::init(
        LevelFilter::Info,
        log_config,
        File::create(format!("{}/apex-map-tracker.log", base_path)).unwrap(),
    )
    .expect("Failed to initialize file logger");
}
