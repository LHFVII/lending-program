use anchor_lang::prelude::*;
use anchor_spl::token_interface::{ self, Mint, TokenAccount, TokenInterface, TransferChecked };
use anchor_spl::associated_token::AssociatedToken;

use crate::state::{PoolConfig, PoolKeyConfig, User, UserDepositAccount};
use crate::error::{LendingProgramError};

#[derive(Accounts)]
#[instruction()]
pub struct DepositCollateral<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut, 
        seeds = [payer.key().as_ref()],
        bump,
    )]  
    pub user_account: Account<'info, User>,

    #[account(
        mut, 
        seeds = [mint.key().as_ref()],
        bump,
    )]  
    pub pool_config: Account<'info, PoolConfig>,

    #[account(
        mut, 
        seeds = [b"treasury", mint.key().as_ref()],
        bump, 
    )]  
    pub pool_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = payer,
        seeds = [user_account.owner.as_ref(), mint.key().as_ref()],
        bump,
        space = 8 + UserDepositAccount::INIT_SPACE,
    )]
    pub user_deposit: Account<'info, UserDepositAccount>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer,
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>, 
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}


impl<'info> DepositCollateral<'info>{
    pub fn deposit_collateral(
        &mut self,
        amount: u64,
        ) -> Result<()>{
            let transfer_cpi_accounts = TransferChecked {
                from: self.user_token_account.to_account_info(),
                mint: self.mint.to_account_info(),
                to: self.pool_token_account.to_account_info(),
                authority: self.payer.to_account_info(),
            };
        
            let cpi_program = self.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, transfer_cpi_accounts);
            let decimals = self.mint.decimals;
            token_interface::transfer_checked(cpi_ctx, amount, decimals)?;

            let pool = &mut self.pool_config;

            if pool.total_deposits == 0 {
                pool.total_deposits = amount;
                pool.total_deposit_shares = amount;
            }
            
            let deposit_ratio = amount.checked_div(pool.total_deposits).unwrap();
            let users_shares = pool.total_deposit_shares.checked_mul(deposit_ratio).unwrap();
            
            let user = &mut self.user_account;
            let mint_address = self.mint.to_account_info().key();
            match mint_address {
                key if key == pool.mint_address => {
                    user.deposited_usdc += amount;
                    user.deposited_usdc_shares += users_shares;
                },
                _ => {
                    return Err(LendingProgramError::UnsupportedMint.into());
                }
            }
            pool.total_deposits += amount;
            pool.total_deposit_shares += users_shares;
            
            
        Ok(())
    }
}