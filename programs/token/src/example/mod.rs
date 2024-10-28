use pinocchio::program_error::ProgramError;

pub mod transfer;

#[derive(Clone, Copy, Debug)]
pub enum TestInstruction {
    Transfer,
}

impl TryFrom<&u8> for TestInstruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            3 => Ok(TestInstruction::Transfer),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}