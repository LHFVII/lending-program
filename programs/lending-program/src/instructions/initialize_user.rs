use anchor_lang::prelude::*;

use crate::state::UserAccount;

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

impl<'info> InitializeUser<'info>{
    pub fn initialize_user(&mut self) -> Result<()>{
        self.user_account.owner = self.payer.key();
        self.user_account.allowed_borrow_amount_in_usdc = 0;
        self.user_account.borrowed_amount_in_usdc = 0;
        Ok(())
    }
}