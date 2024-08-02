use anchor_lang::prelude::*;
use anchor_spl::{
    token::{ Mint, Token, TokenAccount, transfer,Transfer},
    associated_token::{AssociatedToken}
};
use crate::instructions::initialize_user::UserAccount;

use crate::error::{LendingProgramError};


pub fn deposit_collateral<'info>(
    ctx: Context<DepositCollateral>, 
    amount: u64,
    ) -> Result<()>{
        let from = &mut ctx.accounts.user_token_account;
        let to = &mut ctx.accounts.pool_token_account;
        let token_program = &mut ctx.accounts.token_program;
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
        ctx.accounts.user_vault_info.mint = ctx.accounts.deposit_mint.key();
        ctx.accounts.user_vault_info.amount += amount;
        ctx.accounts.user_account.assets.push(ctx.accounts.user_vault_info.key());
    Ok(())
}

#[derive(Accounts)]
pub struct DepositCollateral<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,

    #[account()]
    pub deposit_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = deposit_mint,
        associated_token::authority = payer,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = payer,
        seeds = [b"user_assets".as_ref(), user_token_account.key().as_ref()],
        bump,
        space = 8 + UserAssets::INIT_SPACE,
    )]
    pub user_vault_info: Account<'info, UserAssets>,

    #[account(mut)]
    pub pool_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info,System>
}

#[account]
#[derive(InitSpace)]
pub struct UserAssets {
    pub mint: Pubkey,
    pub amount: u64
}