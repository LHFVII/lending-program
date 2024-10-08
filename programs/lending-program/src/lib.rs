pub mod instructions;
pub mod state;
pub mod error;
pub mod constants;

use anchor_lang::prelude::*;

pub use instructions::*;

declare_id!("77B3AdNp6RRzsVMQSWAUVv28RXmA8YJVAfWkimRTwXi6");

#[program]
pub mod lending_program {
    use super::*;

    pub fn borrow_asset(ctx: Context<BorrowAsset>, amount: u64) -> Result<()> {
        ctx.accounts.borrow_asset(amount)
    }

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        ctx.accounts.initialize_user()
    }

    pub fn initialize_pool(ctx: Context<InitializePool>, liquidation_threshold: u64, max_ltv: u64) -> Result<()> {
        ctx.accounts.initialize_pool(liquidation_threshold, max_ltv)
    }

    pub fn deposit_collateral(ctx: Context<DepositCollateral>, amount: u64) -> Result<()> {
        ctx.accounts.deposit_collateral(amount)
    }

    pub fn withdraw_collateral(ctx: Context<WithdrawCollateral>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw_collateral(amount)
    }
}


