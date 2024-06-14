pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;
pub mod utils;


#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

use solana_program::{declare_id, pubkey::Pubkey};
use state::stamp::Stamp;

declare_id!("cardFRMHxFN4X1urijmqb7gWSMT7bAep4Pd4LuLciG3");

/// Generates deposit program address
pub fn find_stanp_program_address(program_id: &Pubkey, reference: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            Stamp::PREFIX.as_bytes(),
            program_id.as_ref(),
            reference.as_ref(),
        ],
        program_id,
    )
}
