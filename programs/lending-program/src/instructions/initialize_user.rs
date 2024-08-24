use anchor_lang::prelude::*;

use crate::state::User;

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    #[account(
        init,
        payer = payer,
        seeds = [payer.key().as_ref()],
        bump,
        space = 8 + User::INIT_SPACE + 16,
    )]
    pub user_account: Account<'info, User>,
    pub system_program: Program<'info, System>
}

impl<'info> InitializeUser<'info>{
    pub fn initialize_user(&mut self) -> Result<()>{
        self.user_account.owner = self.payer.key();
        let now = Clock::get()?.unix_timestamp; 
        self.user_account.last_updated = now;
        Ok(())
    }
}