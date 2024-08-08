use anchor_lang::prelude::*;
use anchor_spl::{
    token::{ Mint, Token, TokenAccount, transfer,Transfer},
    associated_token::{AssociatedToken}
};
use rust_decimal::{prelude::FromPrimitive, Decimal};
use switchboard_on_demand::PullFeedAccountData;
use crate::instructions::initialize_user::UserAccount;

use crate::error::{LendingProgramError};


pub fn withdraw_collateral<'info>(
    ctx: Context<WithdrawCollateral>, 
    amount: u64,
    ) -> Result<()>{
        let mut requested_withdraw_amount: Decimal;
        let mut token_price_in_usdc: Decimal;

        let feed_account = ctx.accounts.feed.data.borrow();
        let feed = PullFeedAccountData::parse(feed_account).unwrap();

        match feed.value(){
            None => return Err(LendingProgramError::InvalidSwitchboardAccount.into()),
            Some(feed_value) => {token_price_in_usdc = feed_value;}
        };
        match Decimal::from_u64(amount){
            None => return Err(LendingProgramError::InvalidSwitchboardAccount.into()),
            Some(converted) => {requested_withdraw_amount = converted;}
        };

        require!(amount <= ctx.accounts.user_token_account.amount,
            LendingProgramError::NotEnoughFunds);
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
            amount
        );    
        ctx.accounts.user_account.allowed_borrow_amount_in_usdc -= amount /10;
    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawCollateral<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub collateral_mint: Account<'info, Mint>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub feed: AccountInfo<'info>,

    #[account(mut)]
    pub pool_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info,System>
}