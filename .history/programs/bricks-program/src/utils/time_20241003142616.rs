use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock; // Import Clock

pub fn current_timestamp() -> u64 {
    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp;

    current_timestamp
}