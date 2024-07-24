use anchor_lang::prelude::*;

pub fn initialize_asset(
    ctx: Context<InitializeAsset>,
    max_ltv: u64,
    liquidation_threshold: u64, 
    apy:u64) -> Result<()>{
        ctx.asset_config.max_ltv = max_ltv;
        ctx.asset_config.liquidation_threshold = liquidation_threshold;
        ctx.asset_config.apy = apy;
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeAsset<'info> {
    #[account(
        init,
        payer = payer,
        seeds = [payer.key().as_ref()],
        bump,
        space = 8 + AssetAccount::INIT_SPACE,
    )]
    pub asset_config: Account<'info,AssetConfig>,

    pub mint: Account<'info,Mint>,

    #[account(
        init,
        seeds = [mint.key().as_ref()],
        token::mint = mint,
        token::authority = asset_token_account,
        space = 8 + AssetConfig::INIT_SPACE,
        bump
    )]
    pub asset_token_account: Account<'info, TokenAccount>

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info,System>
}

#[account]
#[derive(InitSpace)]
pub struct AssetConfig{
    pub max_ltv: u64,
    pub liquidation_threshold: u64,
    pub apy:u64
}