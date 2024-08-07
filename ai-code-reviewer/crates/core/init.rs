use log::LevelFilter;
use crate::options::Args;

pub fn init(args: &Args) {
    init_log(args.log_level().clone());
}

pub fn init_log(level: LevelFilter) {
    env_logger::builder()
        .filter_level(level)
        .init();
}