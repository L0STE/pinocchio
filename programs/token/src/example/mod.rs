use pinocchio::program_error::ProgramError;

pub mod revoke;
pub mod set_authority;
pub mod sync_native;
pub mod thaw_account;
pub mod transfer_checked;
pub mod transfer;

#[derive(Clone, Copy, Debug)]
pub enum TestInstruction {
    Revoke,
    SyncNative,
    ThawAccount,
    TransferChecked,
    Transfer,
}

impl TryFrom<&u8> for TestInstruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            3 => Ok(TestInstruction::Transfer),
            5 => Ok(TestInstruction::Revoke),
            11 => Ok(TestInstruction::ThawAccount),
            12 => Ok(TestInstruction::TransferChecked),
            17 => Ok(TestInstruction::SyncNative),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}