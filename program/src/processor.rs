use borsh::BorshDeserialize;
use crate::instruction::StampInstruction;

use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

pub mod stamp;

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        msg!("Start deserialize card instruction");
        let instruction = StampInstruction::try_from_slice(instruction_data)?;
        msg!("Successfully deserialized card instruction");

        match instruction {
            StampInstruction::InitStamp(args) => {
                msg!("Instruction: Init Stamp");
                stamp::init(program_id, accounts, args)
            }
        }
    }
}
