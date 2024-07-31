pub mod instructions;
pub mod error;

use anchor_lang::prelude::*;

use instructions::*;

declare_id!("77B3AdNp6RRzsVMQSWAUVv28RXmA8YJVAfWkimRTwXi6");

#[program]
pub mod lending_program {
    use super::*;

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        instructions::initialize_user(ctx)
    }

    pub fn initialize_pool(ctx: Context<InitializePool>, mint: Pubkey) -> Result<()> {
        instructions::initialize_pool(ctx,mint)
    }

    pub fn deposit_collateral(ctx: Context<DepositCollateral>, amount: u64) -> Result<()> {
        instructions::deposit_collateral(ctx, amount)
    }

    pub fn withdraw_collateral(ctx: Context<WithdrawCollateral>, amount: u64) -> Result<()> {
        instructions::withdraw_collateral(ctx, amount)
    }
}


