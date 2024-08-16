use std::ops::Div;

use anchor_lang::prelude::*;
use anchor_spl::{
    token::{ Mint, Token, TokenAccount, transfer,Transfer},
    associated_token::{AssociatedToken}
};
use rust_decimal::{prelude::FromPrimitive,prelude::ToPrimitive, Decimal};
use switchboard_on_demand::PullFeedAccountData;
use crate::state::{UserAccount, UserDepositAccount};

use crate::error::{LendingProgramError};

#[derive(Accounts)]
pub struct DepositCollateral<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account()]
    pub deposit_mint: Account<'info, Mint>,

    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        init_if_needed,
        payer = payer,
        seeds = [user_account.owner.as_ref(), deposit_mint.key().as_ref()],
        bump,
        space = 8 + UserDepositAccount::INIT_SPACE,
    )]
    pub user_deposit: Account<'info, UserDepositAccount>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = deposit_mint,
        associated_token::authority = payer,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub feed: AccountInfo<'info>,

    #[account(mut)]
    pub pool_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info,System>
}


impl<'info> DepositCollateral<'info>{
    pub fn deposit_collateral(
        &mut self,
        amount: u64,
        ) -> Result<()>{
            let feed_account = self.feed.data.borrow();
            let feed = PullFeedAccountData::parse(feed_account).unwrap();
            let mut token_price_in_usdc;
            let mut deposited_amount: Decimal;
            match feed.value(){
                None => return Err(LendingProgramError::InvalidSwitchboardAccount.into()),
                Some(feed_value) => {token_price_in_usdc = feed_value;}
            };
            match Decimal::from_u64(amount){
                None => return Err(LendingProgramError::InvalidSwitchboardAccount.into()),
                Some(converted) => {deposited_amount = converted;}
            };
            let deposited_amount_in_usdc = deposited_amount * token_price_in_usdc;
            
            let from = &mut self.user_token_account;
            let to = &mut self.pool_token_account;
            let token_program = &mut self.token_program;
            let _ = transfer(
                CpiContext::new(
                    token_program.to_account_info(),
                    Transfer{
                        from: from.to_account_info(),
                        to: to.to_account_info(),
                        authority: self.payer.to_account_info(),
                    },
                ),
                amount
            );
            let allowed_borrow_amount_in_usdc: u64;
            match Decimal::to_u64(&deposited_amount_in_usdc){
                None => return Err(LendingProgramError::InvalidSwitchboardAccount.into()),
                Some(converted) => {allowed_borrow_amount_in_usdc = converted;}
            };
            self.user_account.allowed_borrow_amount_in_usdc = allowed_borrow_amount_in_usdc.div(2);
            self.user_deposit.amount += allowed_borrow_amount_in_usdc;
        Ok(())
    }
    
    

}