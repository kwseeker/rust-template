use std::str::FromStr;
use log::LevelFilter;

pub fn init() {
    init_log(None);
}

pub fn init_log(log_level: Option<String>) {
    let log_level = log_level.as_ref()
        .map_or_else(|| LevelFilter::Info, |level| {
            LevelFilter::from_str(level).expect("Invalid log level format")
        });
    env_logger::builder()
        .filter_level(log_level)
        .init();
}