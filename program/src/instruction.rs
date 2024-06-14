//! Instruction types
#![allow(missing_docs)]

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program, sysvar,
};

/// Initialize a funding arguments
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
/// Initialize a funding params
pub struct StampArgs {
    pub bump: u8,
    pub reference: String,
}


#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone,)]
pub enum StampInstruction {
    /// Accounts expected:
    ///
    /// 0. `[signer]` The fee payer
    /// 1. `[writable]` The stamp account, it will hold all necessary info about the transaction.
    /// 2. `[]` The rent sysvar
    /// 3. `[]` The token program
    InitStamp(StampArgs),
}

/// Create `Stamp` instruction
pub fn stamp(
    program_id: &Pubkey,
    payer: &Pubkey,
    stamp: &Pubkey,
    args: StampArgs,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*payer, true),
        AccountMeta::new(*stamp, false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    Instruction::new_with_borsh(*program_id, &StampInstruction::InitStamp(args), accounts)
}
