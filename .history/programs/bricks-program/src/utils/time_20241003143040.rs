use anchor_lang::{prelude::*, solana_program::clock};
use std::convert::TryInto;

pub fn current_timestamp() -> u64 {
    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp.try_into().unwrap();

    current_timestamp
}