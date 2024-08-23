use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{ self, Mint, TokenAccount, TokenInterface, TransferChecked };
use crate::constants::USDC_ADDRESS;
use crate::state::PoolConfig;
use crate::{error::LendingProgramError, state::{User, UserDepositAccount}};

#[derive(Accounts)]
pub struct WithdrawCollateral<'info> {
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

            // FIXME: Change from if statement to match statement?? Use PDA deserialization to get the mint address??
            let mint_address = &self.mint.to_account_info().key().to_string();
            if mint_address == USDC_ADDRESS {
                deposited_value = user.deposited_usdc;
            } else {
                return Err(LendingProgramError::UnsupportedMint.into());
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

            let cpi_program = self.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, transfer_cpi_accounts);
            let decimals = self.mint.decimals;

            token_interface::transfer_checked(cpi_ctx, amount, decimals)?;

            let pool = &mut self.pool;
            let shares_to_remove = (amount as f64 / pool.total_deposits as f64) * pool.total_deposit_shares as f64;

            let user = &mut self.user_account;
            
            if mint_address == USDC_ADDRESS  {
                user.deposited_usdc -= shares_to_remove as u64;
            } else {
                user.deposited_sol -= shares_to_remove as u64;
            }

            pool.total_deposits -= amount;
            pool.total_deposit_shares -= shares_to_remove as u64;

            
        Ok(())
    }
    
    

}
