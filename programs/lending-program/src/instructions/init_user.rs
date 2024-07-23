use anchor_lang::prelude::*;

pub fn initializeUser(ctx: Context<InitializeUser>) -> Result<()>{
    let user: &mut Account<UserAccount> = &mut ctx.accounts.user_account;
    user.total_collateral = 0;
    user.total_borrowed = 0;
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(
        init,
        payer = payer,
        seeds = [payer.key().as_ref()],
        bump,
        space = 8 + UserAccount::INIT_SPACE,
    )]
    pub user_account: Account<'info,UserAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info,System>
}

#[account]
#[derive(InitSpace)]
pub struct UserAccount{
    pub owner: Pubkey,
    pub total_collateral: u64,
    pub total_borrowed:u64
}