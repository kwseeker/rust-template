use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) fn current() -> u64 {
    let current = SystemTime::now().duration_since(UNIX_EPOCH);
    match current {
        Ok(duration) => duration.as_millis() as u64,
        Err(_) => panic!("SystemTime get current timestamp failed"),
    }
}
