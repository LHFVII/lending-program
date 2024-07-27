use anchor_lang::prelude::*;

#[error_code]
pub enum LendingProgramError {
    
    #[msg("The asset proposed to pay is not the same as the liquidity pool")]
    InvalidPoolMint,
}