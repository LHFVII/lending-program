use anchor_lang::{prelude::*, solana_program::address_lookup_table::instruction};
use anchor_spl::{
    token::{ Mint, Token, TokenAccount},
    associated_token::{AssociatedToken}
};

use crate::state::PoolConfig;

#[derive(Accounts)]
#[instruction(pool_number: u64)]
pub struct InitializePool<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,
    
    pub mint: Account<'info, Mint>,
    
    #[account(
        init,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer,
    )]
    pub pool_token_account: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = payer,
        seeds=[mint.key().as_ref(), payer.key().as_ref()],
        space = 8 + PoolConfig::INIT_SPACE,
        bump
    )]
    pub pool_config: Account<'info, PoolConfig>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>
}

impl<'info> InitializePool<'info>{
    pub fn initialize_pool(&mut self, liquidation_threshold: u64, max_ltv: u64) -> Result<()>{
        let pool = &mut self.pool_config;
        pool.mint_address = self.mint.key();
        pool.authority = self.payer.key();
        pool.liquidation_threshold = liquidation_threshold;
        pool.max_ltv = max_ltv;
        Ok(())
    }
}