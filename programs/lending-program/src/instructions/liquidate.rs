use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, transfer, Transfer};
use switchboard_on_demand::on_demand::accounts::pull_feed::PullFeedAccountData;
use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use crate::error::LendingProgramError;
use crate::instructions::initialize_user::UserAccount;

pub fn liquidate(
    ctx: Context<LiquidateUser>
    ) -> Result<()>{
        let feed_account = ctx.accounts.feed.data.borrow();
        let feed = PullFeedAccountData::parse(feed_account).unwrap();
        let borrowed_amount = ctx.accounts.user_account.borrowed_amount_in_usdc;
        let allowed_borrow_amount_in_usdc: Decimal;
        let mut token_price_in_usdc;
        let mut borrowed_amount_decimal: Decimal;
        match feed.value(){
            None => return Err(LendingProgramError::InvalidSwitchboardAccount.into()),
            Some(feed_value) => {token_price_in_usdc = feed_value;}
        };
        match feed.value(){
            None => return Err(LendingProgramError::InvalidSwitchboardAccount.into()),
            Some(feed_value) => {token_price_in_usdc = feed_value;}
        };
        match Decimal::from_u64(borrowed_amount){
            None => return Err(LendingProgramError::InvalidSwitchboardAccount.into()),
            Some(converted) => {borrowed_amount_decimal = converted;}
        };
        match Decimal::from_u64(ctx.accounts.user_account.allowed_borrow_amount_in_usdc){
            None => return Err(LendingProgramError::InvalidSwitchboardAccount.into()),
            Some(converted) => {allowed_borrow_amount_in_usdc = converted;}
        };

        
        let borrowed_amount_in_usdc = borrowed_amount_decimal * token_price_in_usdc;
        require!(borrowed_amount_in_usdc >= allowed_borrow_amount_in_usdc,
            LendingProgramError::MarginNotLargeEnough
        );
        

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