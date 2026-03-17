#![no_std]

use ai_chrono::{TimeZone, Utc};

pub fn create_time() {
    let _ = Utc.with_ymd_and_hms(2019, 1, 1, 0, 0, 0).unwrap();
}
