use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

pub fn current_timestamp() -> u64 {
    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp as u64;

    current_timestamp
}