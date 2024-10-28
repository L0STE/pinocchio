use pinocchio::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey
};

use crate::instructions::Revoke;

pub fn revoke(accounts: &[AccountInfo]) -> ProgramResult {
    let [token, authority, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    Revoke { 
        token, 
        authority
    }.invoke()?;
   
    Ok(())
}
