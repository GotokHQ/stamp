pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;
pub mod utils;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

use solana_program::{declare_id, pubkey::Pubkey};
use state::stamp::Stamp;

declare_id!("G8azYVro1VexvpWMuQzcQyPJxVgAWYp6pyAWQGJvVKHM");

/// Generates deposit program address
pub fn find_stamp_program_address(program_id: &Pubkey, reference: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            Stamp::PREFIX.as_bytes(),
            reference.as_ref(),
        ],
        program_id,
    )
}
