use std::ops::Div;

use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};
use rust_decimal::{prelude::{FromPrimitive, ToPrimitive}, Decimal};
use switchboard_on_demand::PullFeedAccountData;

use crate::error::LendingProgramError;

use super::{UserAccount, UserDepositAccount};

pub fn absorb_loan(
    ctx: Context<AbsorbLoan>
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

        let liquidator_sender_token_account = &mut ctx.accounts.liquidator_sender_token_account;
        let liquidator_receiver_token_account = &mut ctx.accounts.liquidator_receiver_token_account;
        let pool_sender_token_account = &mut ctx.accounts.pool_sender_token_account;
        let pool_receiver_token_account = &mut ctx.accounts.pool_receiver_token_account;
        
        let token_program = &mut ctx.accounts.token_program;
        let loan_amount = ctx.accounts.user_account.borrowed_amount_in_usdc;
        // Buy the loan
        let _ = transfer(
            CpiContext::new(
                token_program.to_account_info(),
                Transfer{
                    from: liquidator_sender_token_account.to_account_info(),
                    to: pool_receiver_token_account.to_account_info(),
                    authority: ctx.accounts.payer.to_account_info(),
                },
            ),
            loan_amount
        );

        // we also need 10% from the collateral as a gift to the liquidator
        let liquidation_reward = borrowed_amount.div(20);
        let mut liquidation_reward_decimal;
        match Decimal::from_u64(liquidation_reward){
            None => return Err(LendingProgramError::InvalidSwitchboardAccount.into()),
            Some(converted) => {liquidation_reward_decimal = converted;}
        };
        let loan_token_amount_decimal = (borrowed_amount_in_usdc + liquidation_reward_decimal).div(token_price_in_usdc);
        let mut loan_token_amount: u64;

        match Decimal::to_u64(&loan_token_amount_decimal){
            None => return Err(LendingProgramError::InvalidSwitchboardAccount.into()),
            Some(converted) => {loan_token_amount = converted;}
        };
        // We give the collateral (i.e SOL)
        let _ = transfer(
            CpiContext::new(
                token_program.to_account_info(),
                Transfer{
                    from: pool_sender_token_account.to_account_info(),
                    to: liquidator_receiver_token_account.to_account_info(),
                    authority: ctx.accounts.payer.to_account_info(),
                },
            ),
            loan_token_amount
        );
        ctx.accounts.user_deposit_account.amount -= loan_token_amount;
        let new_deposit = ctx.accounts.user_deposit_account.amount;
        ctx.accounts.user_account.allowed_borrow_amount_in_usdc  = new_deposit.div(2);
        
    Ok(())
}

#[derive(Accounts)]
pub struct AbsorbLoan<'info> {

    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub user_deposit_account: Account<'info, UserDepositAccount>,

    #[account(mut)]
    pub pool_receiver_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub pool_sender_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub liquidator_sender_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub liquidator_receiver_token_account: Account<'info, TokenAccount>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub feed: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info,System>
}