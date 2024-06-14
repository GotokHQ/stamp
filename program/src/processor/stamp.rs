//! Init pass instruction processing

use crate::{
    instruction::StampArgs, state::{stamp::Stamp, FLAG_ACCOUNT_SIZE}, utils::*
};

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
};


/// Process InitPass instruction
pub fn init(program_id: &Pubkey, accounts: &[AccountInfo], args: StampArgs) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let authority_info = next_account_info(account_info_iter)?;
    let payer_info = next_account_info(account_info_iter)?;
    let stamp_info = next_account_info(account_info_iter)?;
    let reference_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_account_info = next_account_info(account_info_iter)?;

    assert_signer(authority_info)?;

    if stamp_info.lamports() > 0 && !stamp_info.data_is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }
    create_new_account_raw(
        program_id,
        stamp_info,
        rent_info,
        payer_info,
        system_account_info,
        FLAG_ACCOUNT_SIZE,
        &[
            Stamp::PREFIX.as_bytes(),
            reference_info.key.as_ref(),
            &[args.bump],
        ],
    )?;
    let mut stamp = Stamp::unpack_unchecked(&stamp_info.data.borrow())?;
    if stamp.is_initialized() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }
    stamp.is_initialized = true;
    Stamp::pack(stamp, *stamp_info.data.borrow_mut())?;
    Ok(())
}
