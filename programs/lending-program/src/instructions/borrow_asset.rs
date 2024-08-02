use anchor_lang::prelude::*;
use anchor_spl::{
    token::{ Mint, Token, TokenAccount, transfer,Transfer},
    associated_token::{AssociatedToken}
};
use crate::instructions::initialize_user::UserAccount;
use crate::instructions::deposit_collateral::UserAssets;

use crate::error::{LendingProgramError};

pub fn borrow_asset(
    ctx: Context<BorrowAsset>,
    amount: u64
    ) -> Result<()>{
        msg!("{:?}",ctx.accounts.pool_token_account.mint);
        msg!("{:?}",ctx.accounts.borrow_mint);
        //require!(ctx.accounts.pool_token_account.mint.key() == ctx.accounts.borrow_mint.key(),LendingProgramError::MintMismatch);
        msg!("{:?}",amount);

        
    Ok(())
}

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
    pub user_vault_info: Account<'info, UserAssets>,

    #[account(mut)]
    pub pool_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info,System>
}