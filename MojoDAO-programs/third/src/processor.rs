//! Program state processor

use {
    crate::{
        error::RecordError,
        instruction::RecordInstruction,
        state::{Data, RecordData},
    },
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program_error::ProgramError,
        program_pack::IsInitialized,
        pubkey::Pubkey,
        sysvar::{clock::Clock, Sysvar},
        program::invoke,
    },
};

fn check_authority(authority_info: &AccountInfo, expected_authority: &Pubkey) -> ProgramResult {
    if expected_authority != authority_info.key {
        msg!("Incorrect record authority provided");
        return Err(RecordError::IncorrectAuthority.into());
    }
    if !authority_info.is_signer {
        msg!("Record authority signature missing");
        return Err(ProgramError::MissingRequiredSignature);
    }
    Ok(())
}

/// Instruction processor
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = RecordInstruction::try_from_slice(input)?;
    let account_info_iter = &mut accounts.iter();

    match instruction {
        RecordInstruction::Initialize => {
            msg!("RecordInstruction::Initialize");

            let data_info = next_account_info(account_info_iter)?;
            let authority_info = next_account_info(account_info_iter)?;
            let clock_via_sysvar = Clock::get()?;
            msg!("The current time is: {:?}", clock_via_sysvar.unix_timestamp);

            let mut account_data = RecordData::try_from_slice(*data_info.data.borrow())?;
            if account_data.is_initialized() {
                msg!("Record account already initialized");
                return Err(ProgramError::AccountAlreadyInitialized);
            }

            account_data.authority = *authority_info.key;
            account_data.version = RecordData::CURRENT_VERSION;
            account_data
                .serialize(&mut *data_info.data.borrow_mut())
                .map_err(|e| e.into())
        }

        RecordInstruction::Write { offset, data } => {
            msg!("RecordInstruction::Write");
            let data_info = next_account_info(account_info_iter)?;
            let authority_info = next_account_info(account_info_iter)?;
            let account_data = RecordData::try_from_slice(&data_info.data.borrow())?;
            if !account_data.is_initialized() {
                msg!("Record account not initialized");
                return Err(ProgramError::UninitializedAccount);
            }
            check_authority(authority_info, &account_data.authority)?;
            let start = RecordData::WRITABLE_START_INDEX + offset as usize;
            let end = start + data.len();
            let mut decoded_data = Data::try_from_slice(&data).unwrap();
            let clock_via_sysvar = Clock::get()?;
            msg!("The current time is: {:?}", clock_via_sysvar.unix_timestamp);

            // some notes
            // I need to add an error if tier is not an integer, or negative.
            // I also need to add an error if cost is negative
            // duration calculation can be tweeked (or seperated by tier without including in formula)
            // also need to add a check that the updated tier doesn't interfere with current tier.

            let k:u32 = 100000;
            let calculated_duration = (decoded_data.cost / (decoded_data.tier as u32))*k;
            msg!("recieved payment of ${:?} for a tier {:?} subscription.", decoded_data.cost, decoded_data.tier);
            msg!("User recieved {:?} seconds to their subscription.", calculated_duration);

            // now we do not want all the user's data that gets passed in to directly reflect in
            // the account data. The only value that truely matters is cost and tier, which is what
            // the program uses to calculate the duration that the user will be subscribed for.
            // the total is simply derived from the previous total + cost.
            // simply: decoded_data.total and decoded_data.duration are not used when passed in,
            // they are purely derived from cost and tier.
            msg!("The incoming data: {:?}", decoded_data);
            //let bb = Data::try_from_slice(account_data.data).unwrap();
            msg!("current account data: {:?}", account_data.data);

            // this is where I import USDC transfer and Mojo Burn

            decoded_data.total = account_data.data.total + decoded_data.cost;
            if account_data.data.duration < clock_via_sysvar.unix_timestamp as u32 {
                decoded_data.duration = calculated_duration + clock_via_sysvar.unix_timestamp as u32;
            } else {
                decoded_data.duration = calculated_duration + account_data.data.duration;
            }

            msg!("Outgoing data: {:?}", decoded_data);

            let new_data = decoded_data.try_to_vec().unwrap();

            if end > data_info.data.borrow().len() {
                Err(ProgramError::AccountDataTooSmall)
            } else {
                data_info.data.borrow_mut()[start..end].copy_from_slice(&new_data);
                Ok(())
            }
        }

        RecordInstruction::SetAuthority => {
            msg!("RecordInstruction::SetAuthority");
            let data_info = next_account_info(account_info_iter)?;
            let authority_info = next_account_info(account_info_iter)?;
            let new_authority_info = next_account_info(account_info_iter)?;
            let mut account_data = RecordData::try_from_slice(&data_info.data.borrow())?;
            if !account_data.is_initialized() {
                msg!("Record account not initialized");
                return Err(ProgramError::UninitializedAccount);
            }
            check_authority(authority_info, &account_data.authority)?;
            account_data.authority = *new_authority_info.key;
            account_data
                .serialize(&mut *data_info.data.borrow_mut())
                .map_err(|e| e.into())
        }

        RecordInstruction::CloseAccount => {
            msg!("RecordInstruction::CloseAccount");
            let data_info = next_account_info(account_info_iter)?;
            let authority_info = next_account_info(account_info_iter)?;
            let destination_info = next_account_info(account_info_iter)?;
            let mut account_data = RecordData::try_from_slice(&data_info.data.borrow())?;
            if !account_data.is_initialized() {
                msg!("Record not initialized");
                return Err(ProgramError::UninitializedAccount);
            }
            check_authority(authority_info, &account_data.authority)?;
            let destination_starting_lamports = destination_info.lamports();
            let data_lamports = data_info.lamports();
            **data_info.lamports.borrow_mut() = 0;
            **destination_info.lamports.borrow_mut() = destination_starting_lamports
                .checked_add(data_lamports)
                .ok_or(RecordError::Overflow)?;
            account_data.data = Data::default();
            account_data
                .serialize(&mut *data_info.data.borrow_mut())
                .map_err(|e| e.into())
        }
    }
}
