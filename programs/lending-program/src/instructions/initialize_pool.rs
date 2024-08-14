use anchor_lang::{prelude::*, solana_program::address_lookup_table::instruction};
use anchor_spl::{
    token::{ Mint, Token, TokenAccount},
    associated_token::{AssociatedToken}
};

pub fn initialize_pool(ctx: Context<InitializePool>, pool_number: u64) -> Result<()>{
    Ok(())
}

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

#[account]
#[derive(InitSpace)]
pub struct PoolConfig{
    pub max_amount: u64
}