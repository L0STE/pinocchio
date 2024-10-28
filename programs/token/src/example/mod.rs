use pinocchio::program_error::ProgramError;

pub mod initialize_mint_2;
pub mod initialize_mint;
pub mod mint_to_checked;
pub mod mint_to;
pub mod revoke;
pub mod set_authority;
pub mod sync_native;
pub mod thaw_account;
pub mod transfer_checked;
pub mod transfer;

#[derive(Clone, Copy, Debug)]
pub enum TestInstruction {
    InitializeMint2,
    InitializeMint,
    MintToChecked,
    MintTo,
    Revoke,
    // SetAuthority, -> todo after fixing the issue
    SyncNative,
    ThawAccount,
    TransferChecked,
    Transfer,
}

impl TryFrom<&u8> for TestInstruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TestInstruction::InitializeMint),
            3 => Ok(TestInstruction::Transfer),
            5 => Ok(TestInstruction::Revoke),
            7 => Ok(TestInstruction::MintTo),
            11 => Ok(TestInstruction::ThawAccount),
            12 => Ok(TestInstruction::TransferChecked),
            14 => Ok(TestInstruction::MintToChecked),
            17 => Ok(TestInstruction::SyncNative),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}