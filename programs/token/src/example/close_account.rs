use pinocchio::{
    account_info::AccountInfo, ProgramResult, program_error::ProgramError,
};

use crate::instructions::CloseAccount;

pub fn close_account(accounts: &[AccountInfo]) -> ProgramResult {
    
    let [
        token,
        destination,
        authority,
        _token_program,
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    CloseAccount {
        token,
        destination,
        authority,
    }.invoke()?;

    Ok(())
}