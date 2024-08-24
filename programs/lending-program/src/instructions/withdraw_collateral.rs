use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{ self, Mint, TokenAccount, TokenInterface, TransferChecked };
use crate::state::{PoolConfig};
use crate::{error::LendingProgramError, state::{User}};

#[derive(Accounts)]
pub struct WithdrawCollateral<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub user_signer: Signer<'info>,
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
        seeds = [user_signer.key().as_ref()],
        bump,
    )]  
    pub user_account: Account<'info, User>,
    
    #[account(mut)]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>, 
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    
}

impl<'info> WithdrawCollateral<'info> {
    pub fn withdraw_collateral(
        &mut self, 
        amount: u64,
        ) -> Result<()>{
            let user = &mut self.user_account;

            let deposited_value; 

            let mint_address = self.mint.to_account_info().key();
            match mint_address {
                key if key == self.pool.mint_address => {
                    deposited_value = user.deposited_usdc;
                },
                _ => {
                    return Err(LendingProgramError::UnsupportedMint.into());
                }
            }
            if amount > deposited_value {
                return Err(LendingProgramError::NotEnoughFunds.into());
            }
            
            let transfer_cpi_accounts = TransferChecked {
                from: self.pool_token_account.to_account_info(),
                mint: self.mint.to_account_info(),
                to: self.user_token_account.to_account_info(),
                authority: self.payer.to_account_info(),
            };

            msg!("Pool token account owner: {:?}", self.pool_token_account.owner);
            msg!("Pool token account owner: {:?}", self.user_token_account.owner);
            msg!("Expected authority: {:?}", self.payer.key());
            
            
            let cpi_program = self.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, transfer_cpi_accounts);
            let decimals = self.mint.decimals;
            msg!("After CPI");

            token_interface::transfer_checked(cpi_ctx, amount, decimals)?;

            msg!("After checked transfer");

            let pool = &mut self.pool;
            let shares_to_remove = (amount as f64 / pool.total_deposits as f64) * pool.total_deposit_shares as f64;

            let user = &mut self.user_account;
            
            if mint_address == pool.mint_address  {
                user.deposited_usdc -= shares_to_remove as u64;
            } else {
                user.deposited_sol -= shares_to_remove as u64;
            }

            pool.total_deposits -= amount;
            pool.total_deposit_shares -= shares_to_remove as u64;

            
        Ok(())
    }
}
