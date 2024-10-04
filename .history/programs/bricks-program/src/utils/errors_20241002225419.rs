use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("User already exists.")]
    UserAlreadyExists,
    #[msg("Amount requested is more than available amount")]
    InsufficientValue
}