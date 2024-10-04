use anchor_lang::prelude::Clock;

pub fn current_timestamp() -> u64 {
    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp;

    current_timestamp
}