use pinocchio::{
    account_info::AccountInfo, instruction::{AccountMeta, Instruction, Signer}, program::invoke_signed, ProgramResult
};

/// Freeze an Initialized account using the Mint's freeze_authority 
///
/// ### Accounts:
///   0. `[WRITE]` The account to freeze.
///   1. `[]` The token mint.
///   2. `[SIGNER]` The mint freeze authority.
pub struct FreezeAccount<'a> {
    /// Token Account to freeze.
    pub token: &'a AccountInfo,

    /// Mint Account.
    pub mint: &'a AccountInfo,

    /// Mint Freeze Authority Account
    pub freeze_authority: &'a AccountInfo
}

impl<'a> FreezeAccount<'a> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[Signer]) -> ProgramResult {
        // account metadata
        let account_metas: [AccountMeta; 3] = [
            AccountMeta::writable(self.token.key()),
            AccountMeta::readonly(self.mint.key()),
            AccountMeta::readonly_signer(self.freeze_authority.key()),
        ];

        let instruction = Instruction {
            program_id: &crate::ID,
            accounts: &account_metas,
            data: &[10]
        };

        invoke_signed(
            &instruction, 
            &[self.token, self.mint, self.freeze_authority], 
            signers)
    }
}