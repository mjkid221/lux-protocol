use anchor_lang::prelude::*;
use light_sdk::light_program;

pub mod state;
pub use state::*;
pub mod processor;
pub use processor::*;
pub mod constants;
pub use constants::*;
pub mod error;
pub use error::*;

pub use lux_keys_macros::*;

declare_id!("2SmNpccSbqmfxgVhRgu9gsvZyZj1h8FE6ZHLvV5JNiMW");

#[light_program]
#[program]
pub mod lux_session {
    use super::*;

    pub fn create_session<'info>(
        ctx: LightContext<'_, '_, '_, 'info, CreateSessionToken<'info>>,
        top_up_amount_lamports: Option<u64>,
        valid_until: Option<i64>,
    ) -> Result<()> {
        let top_up_amount_lamports: u64 = top_up_amount_lamports.unwrap_or(0);
        let valid_until =
            valid_until.unwrap_or(Clock::get()?.unix_timestamp + DEFAULT_TOKEN_EXPIRATION_SECONDS);

        create_session_token_handler(&mut ctx, top_up_amount_lamports, valid_until)?;
        Ok(())
    }

    pub fn revoke_session<'info>(
        ctx: LightContext<'_, '_, '_, 'info, RevokeSessionToken<'info>>,
    ) -> Result<()> {
        revoke_session_token_handler(&mut ctx)?;
        Ok(())
    }

    pub fn test_session<'info>(
        ctx: LightContext<'_, '_, '_, 'info, TestSession<'info>>,
    ) -> Result<()> {
        test_session_handler(&mut ctx)?;
        Ok(())
    }

    // TODO: Implement
    // Whitelisted Revokers
    // Take % fee during top-up
    // Ideally these should be multi-sig-able
}
