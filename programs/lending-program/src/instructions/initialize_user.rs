use anchor_lang::prelude::*;

pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()>{
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    #[account(
        init,
        payer = payer,
        seeds = [payer.key().as_ref()],
        bump,
        space = 8 + UserAccount::INIT_SPACE,
    )]
    pub user_account: Account<'info,UserAccount>,

    
    pub system_program: Program<'info,System>
}

#[account]
#[derive(InitSpace)]
pub struct UserAccount{
    pub owner: Pubkey,
    pub total_collateral: u64,
    pub total_borrowed:u64
}