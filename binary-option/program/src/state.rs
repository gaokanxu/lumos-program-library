use {
    crate::error::BinaryOptionError,
    borsh::{BorshDeserialize, BorshSerialize},
    lumos_program::{
        account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
        pubkey::Pubkey,
    },
};

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct BinaryOption {
    pub decimals: u8,
    pub circulation: u64,
    pub settled: bool,
    pub escrow_mint_account_pubkey: Pubkey,
    pub escrow_account_pubkey: Pubkey,
    pub long_mint_account_pubkey: Pubkey,
    pub short_mint_account_pubkey: Pubkey,
    pub owner: Pubkey,
    pub winning_side_pubkey: Pubkey,
}

impl BinaryOption {
    pub const LEN: usize = 202;

    pub fn from_account_info(a: &AccountInfo) -> Result<BinaryOption, ProgramError> {
        let binary_option = BinaryOption::try_from_slice(&a.data.borrow_mut())?;
        Ok(binary_option)
    }

    pub fn increment_supply(&mut self, n: u64) -> ProgramResult {
        self.circulation = self
            .circulation
            .checked_add(n)
            .ok_or(BinaryOptionError::AmountOverflow)?;
        Ok(())
    }

    pub fn decrement_supply(&mut self, n: u64) -> ProgramResult {
        if self.circulation < n {
            return Err(BinaryOptionError::InvalidSupply.into());
        }
        self.circulation -= n;
        Ok(())
    }
}
