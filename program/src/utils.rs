//! Program utils

use std::convert::TryInto;

use crate::error::CardError;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_memory::sol_memcmp,
    program_pack::{IsInitialized, Pack},
    pubkey::{Pubkey, PUBKEY_BYTES},
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use spl_token::state::Account;

/// Assert uninitialized
pub fn assert_uninitialized<T: IsInitialized>(account: &T) -> ProgramResult {
    if account.is_initialized() {
        Err(ProgramError::AccountAlreadyInitialized)
    } else {
        Ok(())
    }
}

/// Assert signer
pub fn assert_signer(account: &AccountInfo) -> ProgramResult {
    if account.is_signer {
        return Ok(());
    }

    Err(ProgramError::MissingRequiredSignature)
}

/// Assert owned by
pub fn assert_owned_by(account: &AccountInfo, owner: &Pubkey) -> ProgramResult {
    if account.owner != owner {
        Err(CardError::InvalidOwner.into())
    } else {
        Ok(())
    }
}

/// Assert owned by
pub fn assert_token_owned_by(token: &Account, owner: &Pubkey) -> ProgramResult {
    if token.owner != *owner {
        Err(CardError::InvalidOwner.into())
    } else {
        Ok(())
    }
}

/// Assert account key
pub fn assert_account_key(
    account_info: &AccountInfo,
    key: &Pubkey,
    error: Option<CardError>,
) -> ProgramResult {
    if *account_info.key != *key {
        match error {
            Some(e) => Err(e.into()),
            _ => Err(ProgramError::InvalidArgument),
        }
    } else {
        Ok(())
    }
}

/// Assert account rent exempt
pub fn assert_rent_exempt(rent: &Rent, account_info: &AccountInfo) -> ProgramResult {
    if !rent.is_exempt(account_info.lamports(), account_info.data_len()) {
        Err(ProgramError::AccountNotRentExempt)
    } else {
        Ok(())
    }
}

/// assert initialized account
pub fn assert_initialized<T: Pack + IsInitialized>(
    account_info: &AccountInfo,
) -> Result<T, ProgramError> {
    let account: T = T::unpack_unchecked(&account_info.data.borrow())?;
    if !account.is_initialized() {
        Err(CardError::AccountNotInitialized.into())
    } else {
        Ok(account)
    }
}

/// transfer all the SOL from source to receiver
pub fn empty_account_balance(
    source: &AccountInfo,
    receiver: &AccountInfo,
) -> Result<(), ProgramError> {
    let mut from = source.try_borrow_mut_lamports()?;
    let mut to = receiver.try_borrow_mut_lamports()?;
    **to += **from;
    **from = 0;
    Ok(())
}

pub fn transfer<'a>(
    is_native: bool,
    source_account_info: &AccountInfo<'a>,
    destination_account_info: &AccountInfo<'a>,
    owner_account_info: &AccountInfo<'a>,
    amount: u64,
    signers_seeds: &[&[&[u8]]],
) -> Result<(), ProgramError> {
    if is_native {
        native_transfer(source_account_info, destination_account_info, amount, signers_seeds)
    } else {
        spl_token_transfer(
            source_account_info,
            destination_account_info,
            owner_account_info,
            amount,
            signers_seeds,
        )
    }
}

/// SPL transfer instruction.
pub fn spl_token_transfer<'a>(
    source: &AccountInfo<'a>,
    destination: &AccountInfo<'a>,
    authority: &AccountInfo<'a>,
    amount: u64,
    signers_seeds: &[&[&[u8]]],
) -> Result<(), ProgramError> {
    let ix = spl_token::instruction::transfer(
        &spl_token::id(),
        source.key,
        destination.key,
        authority.key,
        &[],
        amount,
    )?;

    invoke_signed(
        &ix,
        &[source.clone(), destination.clone(), authority.clone()],
        signers_seeds,
    )
}

/// Native instruction.
pub fn native_transfer<'a>(
    source: &AccountInfo<'a>,
    destination: &AccountInfo<'a>,
    amount: u64,
    signers_seeds: &[&[&[u8]]],
) -> Result<(), ProgramError> {
    invoke_signed(
        // for native SOL transfer user_wallet key == user_token_account key
        &system_instruction::transfer(&source.key, &destination.key, amount),
        &[source.clone(), destination.clone()],
        signers_seeds,
    )
}

pub fn calculate_fee(amount: u64, fee_basis_points: u64) -> Result<u64, ProgramError> {
    Ok(amount
        .checked_mul(fee_basis_points)
        .ok_or::<ProgramError>(CardError::MathOverflow.into())?
        .checked_div(10000)
        .ok_or::<ProgramError>(CardError::MathOverflow.into())?)
}

pub fn calculate_amount_with_fee(amount: u64, fee_basis_points: u64) -> Result<u64, ProgramError> {
    Ok(amount
        .checked_add(calculate_fee(amount, fee_basis_points)?)
        .ok_or::<ProgramError>(CardError::MathOverflow.into())?)
}

pub fn create_new_account_raw<'a>(
    program_id: &Pubkey,
    new_account_info: &AccountInfo<'a>,
    rent_sysvar_info: &AccountInfo<'a>,
    payer_info: &AccountInfo<'a>,
    system_program_info: &AccountInfo<'a>,
    size: usize,
    signer_seeds: &[&[u8]],
) -> ProgramResult {
    let rent = &Rent::from_account_info(rent_sysvar_info)?;
    let required_lamports = rent.minimum_balance(size);

    if required_lamports > 0 {
        msg!("Transfer {} lamports to the new account", required_lamports);
        invoke(
            &system_instruction::transfer(&payer_info.key, new_account_info.key, required_lamports),
            &[
                payer_info.clone(),
                new_account_info.clone(),
                system_program_info.clone(),
            ],
        )?;
    }

    let accounts = &[new_account_info.clone(), system_program_info.clone()];

    msg!("Allocate space for the account {}", new_account_info.key);
    invoke_signed(
        &system_instruction::allocate(new_account_info.key, size.try_into().unwrap()),
        accounts,
        &[&signer_seeds],
    )?;

    msg!("Assign the account to the owning program");
    invoke_signed(
        &system_instruction::assign(new_account_info.key, program_id),
        accounts,
        &[&signer_seeds],
    )?;
    Ok(())
}

/// Checks two pubkeys for equality in a computationally cheap way using
/// `sol_memcmp`
pub fn cmp_pubkeys(a: &Pubkey, b: &Pubkey) -> bool {
    sol_memcmp(a.as_ref(), b.as_ref(), PUBKEY_BYTES) == 0
}