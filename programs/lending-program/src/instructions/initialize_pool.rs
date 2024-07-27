use anchor_lang::prelude::*;
use anchor_spl::{
    token::{ Mint, Token, TokenAccount},
    associated_token::{AssociatedToken}
};

pub fn initialize_pool(ctx: Context<InitializePool>) -> Result<()>{
    Ok(())
}

#[derive(Accounts)]
#[derive(Clone)]
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
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>
}