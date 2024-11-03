use anchor_lang::prelude::{
    borsh::{BorshDeserialize, BorshSerialize},
    *,
};

use light_sdk::{compressed_account::LightAccount, light_account};

use crate::SessionError;
use light_hasher::bytes::AsByteVec;

#[light_account]
#[derive(Clone, Debug, Default)]
pub struct SessionToken {
    #[truncate]
    pub authority: Pubkey,
    #[truncate]
    pub target_program: Pubkey,
    #[truncate]
    pub session_signer: Pubkey,
    pub valid_until: i64,
    pub account_status: AccountStatus,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
pub enum AccountStatus {
    Active,
    Stopped,
}

impl AsByteVec for AccountStatus {
    fn as_byte_vec(&self) -> Vec<Vec<u8>> {
        vec![borsh::to_vec(self).unwrap()]
    }
}

impl Default for AccountStatus {
    fn default() -> Self {
        AccountStatus::Active
    }
}

impl IdlBuild for AccountStatus {}

impl SessionToken {
    pub const LEN: usize = 8 + std::mem::size_of::<Self>();
    pub const SEED_PREFIX: &'static str = "lux_session_token";

    fn is_expired(&self) -> Result<bool> {
        let now = Clock::get()?.unix_timestamp;
        Ok(now < self.valid_until)
    }

    // validate the token
    pub fn validate(&self, _: ValidityChecker) -> Result<bool> {
        // Check the account status isn't stopped
        if self.account_status == AccountStatus::Stopped {
            return Err(SessionError::AccountStopped.into());
        }
        // Check if the token has expired
        self.is_expired()
    }
}

pub struct ValidityChecker<'info> {
    pub session_token: LightAccount<SessionToken>,
    pub session_signer: Signer<'info>,
    pub authority: Pubkey,
    pub target_program: Pubkey,
}
