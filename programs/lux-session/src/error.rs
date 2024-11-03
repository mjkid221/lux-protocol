use anchor_lang::prelude::*;

#[error_code]
pub enum SessionError {
    #[msg("Requested validity is too long")]
    ValidityTooLong,
    #[msg("Invalid session token")]
    InvalidToken,
    #[msg("No session token provided")]
    NoToken,
    #[msg("Account is stopped")]
    AccountStopped,
    #[msg("Unauthorized")]
    Unauthorized,
}
