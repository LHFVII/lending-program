use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, transfer, Transfer};
use switchboard_on_demand::on_demand::accounts::pull_feed::PullFeedAccountData;
use rust_decimal::Decimal;
use crate::error::LendingProgramError;
use crate::instructions::initialize_user::UserAccount;

pub fn liquidate(
    ctx: Context<LiquidateUser>
    ) -> Result<()>{
        let feed_account = ctx.accounts.feed.data.borrow();
        let feed = PullFeedAccountData::parse(feed_account).unwrap();
        let mut price;
        match feed.value(){
            None => return Err(LendingProgramError::InvalidSwitchboardAccount.into()),
            Some(feed_value) => {price = feed_value;}
        };
        let borrowed_amount = Decimal::new(ctx.accounts.user_account.borrowed_amount_in_usdc as i64,32);
        let pool_amount = Decimal::new(ctx.accounts.pool_token_account.amount as i64, 32);
        

        let from = &mut ctx.accounts.pool_token_account;
        let to = &mut ctx.accounts.user_token_account;
        let token_program = &mut ctx.accounts.token_program;

        let _ = transfer(
            CpiContext::new(
                token_program.to_account_info(),
                Transfer{
                    from: from.to_account_info(),
                    to: to.to_account_info(),
                    authority: ctx.accounts.payer.to_account_info(),
                },
            ),
            borrowed_amount
        );
    Ok(())
}

#[derive(Accounts)]
pub struct LiquidateUser<'info> {
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub pool_token_account: Account<'info, TokenAccount>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub feed: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info,System>
}