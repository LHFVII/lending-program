use anchor_lang::prelude::*;

pub fn initializeUser(ctx: Context<InitializeUser>) -> Result<()>{
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    payer: Signer<'info>,
    system_program: Program<'info,System>
}