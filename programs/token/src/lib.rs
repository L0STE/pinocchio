#![no_std]

pub mod instructions;
pub mod state;

pinocchio_pubkey::declare_id!("11111111111111111111111111111111");

mod example;
use example::{TestInstruction, transfer::transfer, revoke::revoke, thaw_account::thaw_account, 
    transfer_checked::transfer_checked, sync_native::sync_native, mint_to::mint_to,
    initialize_mint::initialize_mint, mint_to_checked::mint_to_checked, 
    approve_checked::approve_checked, approve::approve, burn_checked::burn_checked, burn::burn, 
    close_account::close_account, freeze_account::freeze_account, initialize_account::initialize_account,
    initialize_account_2::initialize_account_2, initialize_account_3::initialize_account_3,


};

use pinocchio::account_info::AccountInfo;
use pinocchio::entrypoint;
use pinocchio::pubkey::Pubkey;
use pinocchio::ProgramResult;
use pinocchio::program_error::ProgramError;

entrypoint!(process_instruction);

pub const PDA_MARKER: &[u8; 21] = b"ProgramDerivedAddress";

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator, data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match TestInstruction::try_from(discriminator)? {
        TestInstruction::InitializeMint => initialize_mint(accounts, data),
        TestInstruction::Transfer => transfer(accounts, data),
        TestInstruction::Revoke => revoke(accounts),
        TestInstruction::MintTo => mint_to(accounts, data),
        TestInstruction::ThawAccount => thaw_account(accounts),
        TestInstruction::TransferChecked => transfer_checked(accounts, data),
        TestInstruction::MintToChecked => mint_to_checked(accounts, data),
        TestInstruction::SyncNative => sync_native(accounts),
        TestInstruction::ApproveChecked => approve_checked(accounts, data),
        TestInstruction::Approve => approve(accounts, data),
        TestInstruction::BurnChecked => burn_checked(accounts, data),
        TestInstruction::Burn => burn(accounts, data),
        TestInstruction::CloseAccount => close_account(accounts),
        TestInstruction::FreezeAccount => freeze_account(accounts),
        TestInstruction::InitializeAccount => initialize_account(accounts),
        TestInstruction::InitializeAccount2 => initialize_account_2(accounts, data),
        TestInstruction::InitializeAccount3 => initialize_account_3(accounts, data),
    }
}
