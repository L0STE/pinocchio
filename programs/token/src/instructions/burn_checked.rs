use core::mem::MaybeUninit;

use pinocchio::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction, Signer},
    program::invoke_signed,
    ProgramResult,
};

/// Burns tokens by removing them from an account.
///
/// ### Accounts:
///   0. `[WRITE]` The account to burn from.
///   1. `[WRITE]` The token mint.
///   2. `[SIGNER]` The account's owner/delegate.
pub struct BurnChecked<'a> {
    /// Source of the Burn Account
    pub token: &'a AccountInfo,
    /// Mint Account
    pub mint: &'a AccountInfo,
    /// Owner of the Token Account
    pub authority: &'a AccountInfo,
    /// Amount
    pub amount: u64,
    /// Decimals
    pub decimals: u8,
}

impl<'a> BurnChecked<'a> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[Signer]) -> ProgramResult {
        // Account metadata
        let account_metas: [AccountMeta; 3] = [
            AccountMeta::writable(self.token.key()),
            AccountMeta::writable(self.mint.key()),
            AccountMeta::readonly_signer(self.authority.key()),
        ];

        // Instruction data
        // -  [0]: instruction discriminator (1 byte, u8)
        // -  [1..9]: amount (8 bytes, u64)
        // -  [9]: decimals (1 byte, u8)
        let mut instruction_data = MaybeUninit::<[u8; 10]>::uninit();

        // Data population
        unsafe {
            let ptr = instruction_data.as_mut_ptr() as *mut u8;
            // Set discriminator as u8 at offset [0]
            *ptr = 15;
            // Set amount as u64 at offset [1..9]
            *(ptr.add(1) as *mut u64) = self.amount;
            // Set decimals as u8 at offset [9]
            *ptr.add(9) = self.decimals;
        }

        let instruction = Instruction {
            program_id: &crate::ID,
            accounts: &account_metas,
            data: unsafe { &instruction_data.assume_init() },
        };

        invoke_signed(
            &instruction,
            &[self.token, self.mint, self.authority],
            signers,
        )
    }
}
