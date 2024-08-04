use anchor_lang::prelude::*;

#[error_code]
pub enum LendingProgramError {
    
    #[msg("The asset proposed to pay is not the same as the liquidity pool")]
    InvalidPoolMint,

    #[msg("The user account is invalid")]
    InvalidUserAssetAccount,

    #[msg("The amount you provide is less than what you own.")]
    NotEnoughFunds,

    #[msg("The mints have to match.")]
    MintMismatch,

    #[msg("Not a valid Switchboard account")]
    InvalidSwitchboardAccount,
    
    #[msg("Switchboard feed has not been updated in 5 minutes")]
    StaleFeed,
    
    #[msg("Switchboard feed exceeded provided confidence interval")]
    ConfidenceIntervalExceeded,
}