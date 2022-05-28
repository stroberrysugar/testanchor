use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount};

use crate::state::lockup::Lockup;

use super::account_data::Vesting;

#[derive(Accounts)]
pub struct SetupLockup<'info> {
    #[account(init, payer = authority, space = 10)] // TODO: Update space
    pub lockup: Account<'info, Lockup>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateVesting<'info> {
    // Vesting.
    #[account(zero)]
    pub vesting: Account<'info, Vesting>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    // Depositor.
    #[account(mut)]
    pub depositor: AccountInfo<'info>,
    #[account(signer)]
    pub depositor_authority: AccountInfo<'info>,
    // Misc.
    #[account(constraint = token_program.key == &token::ID)]
    pub token_program: AccountInfo<'info>,
    pub clock: Sysvar<'info, Clock>,
}
