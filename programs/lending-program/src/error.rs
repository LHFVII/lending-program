use anchor_lang::prelude::*;

#[error_code]
pub enum LendingProgramError {
    
    #[msg("The asset proposed to pay is not the same as the liquidity pool")]
    InvalidPoolMint,

    #[msg("The amount you provide is less than what you own.")]
    NotEnoughFunds,

    #[msg("The mints have to match.")]
    MintMismatch,
}