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

    pub fn initialize_asset(ctx: Context<InitializeAsset>,max_ltv: u64, liquidation_threshold:u64, apy:u64) -> Result<()> {
        instructions::initialize_asset(ctx, max_ltv, liquidation_threshold, apy)
    }

    pub fn initialize_pool(ctx: Context<InitializePool>) -> Result<()> {
        instructions::initialize_pool(ctx)
    }
}


