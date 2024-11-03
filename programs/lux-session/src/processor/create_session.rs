use anchor_lang::{prelude::*, system_program};

use crate::SessionToken;
use crate::*;
use light_sdk::{
    compressed_account::LightAccount, context::LightContext, light_account, light_accounts,
    merkle_context::PackedAddressMerkleContext,
};
#[light_accounts]
pub struct CreateSessionToken<'info> {
    #[account(mut)]
    #[fee_payer]
    pub payer: Signer<'info>,
    #[self_program]
    pub self_program: Program<'info, crate::program::LuxSession>,
    /// CHECK: Checked in light-system-program.
    #[authority]
    pub cpi_signer: AccountInfo<'info>,
    #[light_account(
        init,
        seeds = [SessionToken::SEED_PREFIX.as_bytes(), target_program.key().as_ref(), session_signer.key().as_ref(), payer.key().as_ref()]
    )]
    pub session_token: LightAccount<SessionToken>,

    #[account(mut)]
    pub session_signer: Signer<'info>,

    /// CHECK the target program is actually a program.
    #[account(executable)]
    pub target_program: AccountInfo<'info>,
}

pub fn create_session_token_handler<'info>(
    ctx: &mut LightContext<'_, '_, '_, 'info, CreateSessionToken<'info>, LightCreateSessionToken>,
    top_up_amount_lamports: u64,
    valid_until: i64,
) -> Result<()> {
    ctx.light_accounts.session_token.valid_until = valid_until;
    ctx.light_accounts.session_token.authority = ctx.accounts.payer.key();
    ctx.light_accounts.session_token.target_program = ctx.accounts.target_program.key();
    ctx.light_accounts.session_token.session_signer = ctx.accounts.session_signer.key();

    if top_up_amount_lamports > 0 {
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.payer.to_account_info(),
                    to: ctx.accounts.session_signer.to_account_info(),
                },
            ),
            top_up_amount_lamports,
        )?;
    }

    Ok(())
}
