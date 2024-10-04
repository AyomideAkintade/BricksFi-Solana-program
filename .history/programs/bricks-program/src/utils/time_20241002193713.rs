use std::time::{SystemTime, UNIX_EPOCH};

pub fn current_timestamp() {
    let now = SystemTime::now();
    let duration = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let seconds: u64 = duration.as_secs(); // This is u64
    println!("Current timestamp in seconds: {}", seconds);
}