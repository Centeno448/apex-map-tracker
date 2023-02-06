use simplelog::{Config, LevelFilter, WriteLogger};
use std::env;
use std::fs::File;

pub fn init_logger() {
    let base_path = env::var("AMC_BASE_LOG_PATH").unwrap();

    WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create(format!("{}/apex-map-tracker.log", base_path)).unwrap(),
    )
    .expect("Failed to initialize logger");
}
