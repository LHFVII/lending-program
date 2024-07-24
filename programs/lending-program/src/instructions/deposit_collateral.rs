use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Token, TokenAccount}
};
use crate::instructions::initialize_user::UserAccount;


pub fn deposit_collateral(
    ctx: Context<DepositCollateral>,
    ) -> Result<()>{
        
    Ok(())
}

#[derive(Accounts)]
pub struct DepositCollateral<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub user_token_account: Account<'info,TokenAccount>,

    #[account(mut)]
    pub asset_pool: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
    
    //pub asset_info: Account<'info, AssetConfig>
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info,System>
}