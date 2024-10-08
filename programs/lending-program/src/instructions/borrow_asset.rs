use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{ self, Mint, TokenAccount, TokenInterface, TransferChecked };
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};
use crate::constants::{MAXIMUM_AGE, SOL_USD_FEED_ID, USDC_ADDRESS, USDC_USD_FEED_ID};
use crate::{error::LendingProgramError, state::{PoolConfig, User}};

#[derive(Accounts)]
pub struct BorrowAsset<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut, 
        seeds = [mint.key().as_ref()],
        bump,
    )]  
    pub pool: Account<'info, PoolConfig>,
    #[account(
        mut, 
        seeds = [b"treasury", mint.key().as_ref()],
        bump, 
    )]  
    pub pool_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut, 
        seeds = [payer.key().as_ref()],
        bump,
    )]  
    pub user_account: Account<'info, User>,
    #[account( 
        init_if_needed, 
        payer = payer,
        associated_token::mint = mint, 
        associated_token::authority = payer,
        associated_token::token_program = token_program,
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>, 
    pub price_update: Account<'info, PriceUpdateV2>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> BorrowAsset<'info> {
    pub fn borrow_asset(
        &mut self,
        amount: u64
        ) -> Result<()>{
            let pool = &mut self.pool;
            let user = &mut self.user_account;
        
            let price_update = &mut self.price_update;
        
            let sol_feed_id = get_feed_id_from_hex(SOL_USD_FEED_ID)?; 
            let usdc_feed_id = get_feed_id_from_hex(USDC_USD_FEED_ID)?;
        
            let sol_price = price_update.get_price_no_older_than(&Clock::get()?, MAXIMUM_AGE, &sol_feed_id)?;
            let usdc_price = price_update.get_price_no_older_than(&Clock::get()?, MAXIMUM_AGE, &usdc_feed_id)?;
            
            let total_collateral = (sol_price.price as u64 * user.deposited_sol) + (usdc_price.price as u64 * user.deposited_usdc);
            let total_borrowed = (sol_price.price as u64 * user.borrowed_sol) + (usdc_price.price as u64 * user.borrowed_usdc);    
        
            let borrowable_amount = (total_collateral as u64 * pool.max_ltv) - total_borrowed;
            require!(borrowable_amount > amount,
                LendingProgramError::OverBorrowableAmount
            );
        
            let safe_borrowable_amount = (total_collateral * pool.liquidation_threshold) - total_borrowed;
        
            if safe_borrowable_amount < amount {
                msg!("Warning: Borrowing above the safe borrowable amount, risk of liquidation may increase.");
            }
        
            let transfer_cpi_accounts = TransferChecked {
                from: self.pool_token_account.to_account_info(),
                mint: self.mint.to_account_info(),
                to: self.user_token_account.to_account_info(),
                authority: self.payer.to_account_info(),
            };
        
            let cpi_program = self.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, transfer_cpi_accounts);
            let decimals = self.mint.decimals;
        
            token_interface::transfer_checked(cpi_ctx, amount, decimals)?;
        
            let borrow_ratio = amount.checked_div(pool.total_borrowed).unwrap();
            let users_shares = pool.total_borrowed_shares.checked_mul(borrow_ratio).unwrap();
        
            pool.total_borrowed += amount;
            pool.total_borrowed_shares += users_shares; 
            let mint_address = &self.mint.to_account_info().key().to_string();
            match mint_address {
                key if key == USDC_ADDRESS => {
                    user.borrowed_usdc += amount;
                    user.deposited_usdc_shares += users_shares;
                },
                _ => {
                    user.borrowed_sol += amount;
                    user.deposited_sol_shares += users_shares;
                }
            }
            Ok(())
    }
}


