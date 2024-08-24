use anchor_lang::prelude::*;

#[error_code]
pub enum LendingProgramError {
    
    #[msg("The asset proposed to pay is not the same as the liquidity pool")]
    InvalidPoolMint,

    #[msg("The user account is invalid")]
    InvalidUserAssetAccount,

    #[msg("The amount you provide is less than what you own.")]
    NotEnoughFunds,

    #[msg("The margin amount is not big enough.")]
    MarginNotLargeEnough,

    #[msg("The mints have to match.")]
    MintMismatch,

    #[msg("This mint is not supported by the protocol.")]
    UnsupportedMint,

    #[msg("Pool is not undercollateralized.")]
    NotUndercollateralized,

    #[msg("The amount entered is above the healthy level")]
    OverBorrowableAmount,

}