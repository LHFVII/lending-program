use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Token}
};
use anchor_spl::token_interface::{Mint, TokenAccount};

pub fn initialize_asset(
    ctx: Context<InitializeAsset>,
    max_ltv: u64,
    liquidation_threshold: u64, 
    apy:u64) -> Result<()>{
        ctx.accounts.asset_config.max_ltv = max_ltv;
        ctx.accounts.asset_config.liquidation_threshold = liquidation_threshold;
        ctx.accounts.asset_config.apy = apy;
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeAsset<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        seeds = [mint.key().as_ref(), payer.key().as_ref()],
        bump,
        space = 8 + AssetConfigAccount::INIT_SPACE,
    )]
    pub asset_config: Account<'info,AssetConfigAccount>,

    pub mint: InterfaceAccount<'info,Mint>,

    #[account(
        init,
        payer = payer,
        seeds = [mint.key().as_ref()],
        token::mint = mint,
        token::authority = asset_token_account,
        bump
    )]
    pub asset_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

#[account]
#[derive(InitSpace)]
pub struct AssetConfigAccount{
    pub max_ltv: u64,
    pub liquidation_threshold: u64,
    pub apy:u64
}


// Currently single collateral
// Multiple collaterals -> Pools
