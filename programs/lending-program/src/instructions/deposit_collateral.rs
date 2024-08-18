use anchor_lang::prelude::*;
use anchor_spl::{
    token::{ Mint, Token, TokenAccount, transfer,Transfer},
    associated_token::{AssociatedToken}
};
use pyth_solana_receiver_sdk::{price_update::{PriceUpdateV2}};
use pyth_sdk_solana::state::SolanaPriceAccount;

use crate::state::{UserAccount, UserDepositAccount};

use crate::error::{LendingProgramError};

#[derive(Accounts)]
#[instruction()]
pub struct DepositCollateral<'info> {
    
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub price_feed: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account()]
    pub deposit_mint: Account<'info, Mint>,

    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        init_if_needed,
        payer = payer,
        seeds = [user_account.owner.as_ref(), deposit_mint.key().as_ref()],
        bump,
        space = 8 + UserDepositAccount::INIT_SPACE,
    )]
    pub user_deposit: Account<'info, UserDepositAccount>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = deposit_mint,
        associated_token::authority = payer,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub pool_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info,System>
}


impl<'info> DepositCollateral<'info>{
    pub fn deposit_collateral(
        &mut self,
        amount: u64,
        ) -> Result<()>{
            let price_feed_account_data: &[u8] = &mut self.price_feed.try_borrow_data()?;
            //msg!("Full account data: {:?}", price_feed_account_data);
            msg!("Account discriminator: {:?}", &price_feed_account_data[0..8]);
            let inbetween: &mut &[u8] = &mut price_feed_account_data.clone();
            let oracle_price;
            match PriceUpdateV2::try_deserialize(&mut &price_feed_account_data[..]) {
                Ok(account) => {
                    oracle_price = account.price_message.price;
                },
                Err(e) => {
                    msg!("Deserialization error: {:?}", e);
                    return Err(ProgramError::InvalidAccountData.into());
                }
            }
            let deposited_amount_in_usdc = amount as i64 * oracle_price;
            
            let from = &mut self.user_token_account;
            let to = &mut self.pool_token_account;
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
            msg!("{}",deposited_amount_in_usdc);
            /*self.user_account.allowed_borrow_amount_in_usdc = allowed_borrow_amount_in_usdc.div(2);
            self.user_deposit.amount += allowed_borrow_amount_in_usdc;*/
        Ok(())
    }
}