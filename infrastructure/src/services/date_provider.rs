use std::time::{SystemTime, UNIX_EPOCH};

use application::common::interfaces::IDateProvider;

pub struct DateTimeProvider;

impl IDateProvider for DateTimeProvider {
    fn now(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
