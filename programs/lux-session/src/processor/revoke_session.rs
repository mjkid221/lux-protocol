use anchor_lang::prelude::*;

use crate::SessionToken;
use crate::*;
use light_sdk::{
    compressed_account::LightAccount, context::LightContext, light_account, light_accounts,
    merkle_context::PackedAddressMerkleContext,
};

#[light_accounts]
pub struct RevokeSessionToken<'info> {
    #[account(mut)]
    #[fee_payer]
    pub payer: Signer<'info>,
    #[self_program]
    pub self_program: Program<'info, crate::program::LuxSession>,
    /// CHECK: Checked in light-system-program.
    #[authority]
    pub cpi_signer: AccountInfo<'info>,

    #[light_account(
        close,
        seeds = [SessionToken::SEED_PREFIX.as_bytes(), session_token.target_program.key().as_ref(), session_token.session_signer.key().as_ref(), session_token.authority.key().as_ref()]
        constraint = session_token.authority == payer.key() @ SessionError::Unauthorized
    )]
    pub session_token: LightAccount<SessionToken>,
}

pub fn revoke_session_token_handler<'info>(
    _: &mut LightContext<'_, '_, '_, 'info, RevokeSessionToken<'info>, LightRevokeSessionToken>,
) -> Result<()> {
    Ok(())
}
