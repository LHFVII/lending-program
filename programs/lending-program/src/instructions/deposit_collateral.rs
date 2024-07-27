use anchor_lang::prelude::*;
use anchor_spl::{
    token::{ Mint, Token, TokenAccount, transfer,Transfer},
    associated_token::{AssociatedToken}
};
use crate::instructions::initialize_user::UserAccount;
use crate::instructions::initialize_pool::InitializePool;

use crate::error::{LendingProgramError};


pub fn deposit_collateral<'info>(
    ctx: Context<DepositCollateral>, 
    amount: u64,
    ) -> Result<()>{
        let from = &mut ctx.accounts.user_token_account;
        let to = &mut ctx.accounts.pool_token_account;
        let token_program = &mut ctx.accounts.token_program;
        msg!("what is going on?");
        transfer(
            CpiContext::new(
                token_program.to_account_info(),
                Transfer{
                    from: from.to_account_info(),
                    to: to.to_account_info(),
                    authority: ctx.accounts.payer.to_account_info(),
                },
            ),
            amount
        );
    Ok(())
}

#[derive(Accounts)]
pub struct DepositCollateral<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        constraint = !deposit_mint.key().eq(&pool.accounts.mint.key()) @ LendingProgramError::InvalidPoolMint
    )]
    pub deposit_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = deposit_mint,
        associated_token::authority = payer,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub pool: Account<'info, InitializePool>,

    #[account(mut)]
    pub pool_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info,System>

}