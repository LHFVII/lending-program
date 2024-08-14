pub mod instructions;
pub mod state;
pub mod error;

use anchor_lang::prelude::*;

use instructions::*;

declare_id!("77B3AdNp6RRzsVMQSWAUVv28RXmA8YJVAfWkimRTwXi6");

#[program]
pub mod lending_program {
    use super::*;

    pub fn borrow_asset(ctx: Context<BorrowAsset>, amount: u64) -> Result<()> {
        instructions::borrow_asset(ctx, amount)
    }

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        instructions::initialize_user(ctx)
    }

    pub fn initialize_pool(ctx: Context<InitializePool>, pool_number: u64) -> Result<()> {
        instructions::initialize_pool(ctx, pool_number)
    }

    pub fn deposit_collateral(ctx: Context<DepositCollateral>, amount: u64) -> Result<()> {
        instructions::deposit_collateral(ctx, amount)
    }

    pub fn withdraw_collateral(ctx: Context<WithdrawCollateral>, amount: u64) -> Result<()> {
        instructions::withdraw_collateral(ctx, amount)
    }
}


