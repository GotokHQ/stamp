pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;
pub mod utils;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

use solana_program::declare_id;


declare_id!("DkhsdcwKejLofq1VhehWdfua6gZweAvvog8RaMPxwbt");
