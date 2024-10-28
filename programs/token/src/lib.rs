#![no_std]

pub mod instructions;
pub mod state;

pinocchio_pubkey::declare_id!("11111111111111111111111111111111");

mod cpi;
use cpi::transfer::transfer;
use cpi::TestInstruction;

use pinocchio::account_info::AccountInfo;
use pinocchio::entrypoint;
use pinocchio::pubkey::Pubkey;
use pinocchio::{entrypoint::ProgramResult, program_error::ProgramError};

entrypoint!(process_instruction);

pub const PDA_MARKER: &[u8; 21] = b"ProgramDerivedAddress";

pub const ID: [u8; 32] =
    five8_const::decode_32_const("22222222222222222222222222222222222222222222");

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator, data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match TestInstruction::try_from(discriminator)? {
        TestInstruction::Tranfer => transfer(accounts, data),
    }
}
