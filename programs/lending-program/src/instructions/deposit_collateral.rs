use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::TokenAccount;

pub fn deposit_collateral(
    ctx: Context<DepositCollateral>,
    ) -> Result<()>{
        
    Ok(())
}

#[derive(Accounts)]
pub struct DepositCollateral<'info> {
    #[account(mut)]
    pub user_token_account: Account<'info,TokenAccount>,
    pub mint: Account<'info,Mint>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info,System>
}