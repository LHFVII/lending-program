use anchor_lang::prelude::*;


pub fn initialize_pool(ctx: Context<InitializePool>) -> Result<()>{
    Ok(())
}

#[derive(Accounts)]
pub struct InitializePool<'info>{
    pub signer: Signer<'info>,
    pub token_account: Account<'info, Token>,
    pub system_program: Program<'info, System>
}