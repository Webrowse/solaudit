use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

/// Instruction layout: no instruction data needed — any call increments the counter.
///
/// Accounts:
///   0. `[writable]` counter — must be owned by this program, 8 bytes of data (u64 LE).
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let iter = &mut accounts.iter();
    let counter = next_account_info(iter)?;

    if counter.owner != program_id {
        msg!("Counter account must be owned by this program");
        return Err(ProgramError::IncorrectProgramId);
    }

    if !counter.is_writable {
        msg!("Counter account must be writable");
        return Err(ProgramError::InvalidAccountData);
    }

    let mut data = counter.try_borrow_mut_data()?;
    if data.len() < 8 {
        msg!("Account data too small for u64 counter (need 8 bytes)");
        return Err(ProgramError::InvalidAccountData);
    }

    let mut value = u64::from_le_bytes(data[..8].try_into().unwrap());
    value = value
        .checked_add(1)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    data[..8].copy_from_slice(&value.to_le_bytes());

    msg!("Counter incremented to {}", value);
    Ok(())
}
