use anchor_lang::prelude::*;
use anchor_spl::{
    token::{ Mint, Token, TokenAccount, transfer, Transfer},
    associated_token::AssociatedToken
};
use pyth_sdk_solana::state::SolanaPriceAccount;
use crate::{error::LendingProgramError, state::UserAccount};



#[derive(Accounts)]
pub struct BorrowAsset<'info> {
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,

    #[account()]
    pub borrow_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = borrow_mint,
        associated_token::authority = payer
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub pool_token_account: Account<'info, TokenAccount>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub price_feed: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info,System>
}

impl<'info> BorrowAsset<'info> {
    pub fn borrow_asset(
        &mut self,
        amount: u64
        ) -> Result<()>{
            let oracle_price;
            match SolanaPriceAccount::account_info_to_feed(&self.price_feed) {
                Ok(account) => {
                    let price_feed = account.get_ema_price_unchecked();
                    let pricer: f64 = price_feed.price as f64;
                    let base: f64 = 10.0;
                    let comp: f64 = base.powi(price_feed.expo);
                    oracle_price = (pricer * comp) as u64;
                },
                Err(e) => {
                    msg!("Deserialization error: {:?}", e);
                    return Err(ProgramError::InvalidAccountData.into());
                }
            }
            let requested_borrow_amount_in_usdc = amount * oracle_price;
            
            
            require!(requested_borrow_amount_in_usdc <= self.pool_token_account.amount,
                LendingProgramError::NotEnoughFunds
            );
            require!(self.user_account.allowed_borrow_amount_in_usdc < requested_borrow_amount_in_usdc,
                LendingProgramError::NotEnoughFunds
            );
            let from = &mut self.pool_token_account;
            let to = &mut self.user_token_account;
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
            self.user_account.borrowed_amount_in_usdc += requested_borrow_amount_in_usdc;
        Ok(())
    }
}


