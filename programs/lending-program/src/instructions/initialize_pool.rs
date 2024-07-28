use anchor_lang::prelude::*;
use anchor_spl::{
    token::{ Mint, Token, TokenAccount},
    associated_token::{AssociatedToken}
};

pub fn initialize_pool(ctx: Context<InitializePool>, mint: Pubkey) -> Result<()>{
    ctx.accounts.pool_config.mint = mint;
    Ok(())
}

#[derive(Accounts)]
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
        space = 8 + PoolConfig::INIT_SPACE,
        seeds =[b"pool".as_ref(), mint.key().as_ref()],
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
    pub mint: Pubkey
}