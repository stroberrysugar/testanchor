mod access_control;
mod account_data;
mod instructions;
mod state;

use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_lang::solana_program::instruction::Instruction;
use anchor_spl::token::{self, TokenAccount, Transfer};

use crate::instructions::*;
use crate::state::lockup::*;

declare_id!("EGfzF2kLJmiRTViZMdgCKT38cPwydaSBvEy6Nc3FZAHE");

#[program]
pub mod testanchor {
    use super::*;

    pub fn initialize_lockup(ctx: Context<SetupLockup>) -> Result<()> {
        ctx.accounts.lockup.initialize(ctx.accounts.authority.key());
        Ok(())
    }
}
