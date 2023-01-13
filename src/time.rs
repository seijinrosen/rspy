use std::{thread, time::Duration};

pub fn sleep(secs: u64) {
    thread::sleep(Duration::from_secs(secs));
}
