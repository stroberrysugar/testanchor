use anchor_lang::accounts::state::ProgramState;
use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_lang::solana_program::instruction::Instruction;
use anchor_spl::token::{self, TokenAccount, Transfer};

#[account]
pub struct Lockup {
    authority: Pubkey,
    whitelist: [Option<()>; 10],
}

impl Lockup {
    pub fn initialize(&mut self, authority: Pubkey) {
        self.authority = authority;
        self.whitelist = [None; 10];
    }
}
