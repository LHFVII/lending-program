use anchor_lang::prelude::*;
use anchor_spl::token_interface::{ Mint, TokenAccount, TokenInterface };

use crate::state::PoolConfig;

#[derive(Accounts)]
#[instruction(pool_number: u64)]
pub struct InitializePool<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,
    
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init, 
        space = 8 + PoolConfig::INIT_SPACE, 
        payer = payer,
        seeds = [mint.key().as_ref()],
        bump, 
    )]
    pub pool_config: Account<'info, PoolConfig>,
    
    #[account(
        init, 
        token::mint = mint, 
        token::authority = pool_token_account,
        payer = payer,
        seeds = [b"treasury", mint.key().as_ref()],
        bump,
    )]
    pub pool_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>, 
    pub system_program: Program <'info, System>,
}

impl<'info> InitializePool<'info>{
    pub fn initialize_pool(&mut self, liquidation_threshold: u64, max_ltv: u64) -> Result<()>{
        let pool = &mut self.pool_config;
        pool.mint_address = self.mint.key();
        pool.authority = self.payer.key();
        pool.liquidation_threshold = liquidation_threshold;
        pool.max_ltv = max_ltv;
        let now = Clock::get()?.unix_timestamp; 
        pool.last_updated = now;
        Ok(())
    }
}