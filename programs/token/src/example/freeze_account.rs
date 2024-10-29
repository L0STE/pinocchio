use pinocchio::{
    account_info::AccountInfo, ProgramResult, program_error::ProgramError,
};

use crate::instructions::FreezeAccount;

pub fn freeze_account(accounts: &[AccountInfo]) -> ProgramResult {

    let [
        token,
        mint,
        freeze_authority,
        _token_program,
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    FreezeAccount {
        token,
        mint,
        freeze_authority,
    }.invoke()?;

    Ok(())
}