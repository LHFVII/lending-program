use anchor_lang::prelude::*;

pub fn initializeAsset(ctx: Context<InitializeAsset>) -> Result<()>{
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
    pub asset_account: Account<'info,AssetAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info,System>
}

#[account]
#[derive(InitSpace)]
pub struct AssetAccount{
    pub max_ltv: u64,
    pub liquidation_threshold: u64,
    pub apy:u64
}