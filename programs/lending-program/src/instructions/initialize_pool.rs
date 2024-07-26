use anchor_lang::prelude::*;


pub fn initialize_pool(ctx: Context<InitializePool>) -> Result<()>{
    Ok(())
}

#[derive(Accounts)]
pub struct InitializePool<'info>{
    pub signer: Signer<'info>,
    
    pub mint: Account<'info, Mint>,
    
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = pool,
    )]
    pub pool_token_account: Account<'info, token::TokenAccount>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    pub system_program: Program<'info, System>
}