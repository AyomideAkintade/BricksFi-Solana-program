use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("User already exists.")]
    UserAlreadyExists,
    #[msg("Amount requested is more than available amount")]
    InsufficientValue,
    #[msg("Time provided has passed")]
    InvalidEndDate,
    #[msg("Invalid account")]
    InvalidAccount
}