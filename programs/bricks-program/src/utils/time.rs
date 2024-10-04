use anchor_lang::prelude::*;
use std::convert::TryInto;

pub fn current_timestamp() -> Result<u64> {
    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp.try_into().unwrap();

    Ok(current_timestamp)
}