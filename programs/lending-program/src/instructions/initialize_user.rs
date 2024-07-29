use anchor_lang::prelude::*;

pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()>{
    ctx.accounts.user_account.owner = ctx.accounts.payer.key();
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
        space = 8 + UserAccount::INIT_SPACE + 16,
    )]
    pub user_account: Account<'info, UserAccount>,
    pub system_program: Program<'info, System>
}

#[account]
#[derive(InitSpace)]
pub struct UserAccount{
    pub owner: Pubkey,
    #[max_len(32)]
    pub assets: Vec<Pubkey>
}


