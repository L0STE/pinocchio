use pinocchio::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey
};

use crate::instructions::SyncNative;

pub fn sync_native(accounts: &[AccountInfo]) -> ProgramResult {
    let [native_token, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    SyncNative { 
        native_token 
    }.invoke()?;
   
    Ok(())
}
