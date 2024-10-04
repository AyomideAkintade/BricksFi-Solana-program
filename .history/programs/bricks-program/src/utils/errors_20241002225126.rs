#[error_code]
pub enum ErrorCode {
    #[msg("User already exists.")]
    UserAlreadyExists,
}