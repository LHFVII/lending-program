use anchor_lang::prelude::*;

pub fn supply_collateral(
    ctx: Context<SupplyCollateral>,
    ) -> Result<()>{
        
    Ok(())
}

#[derive(Accounts)]
pub struct SupplyCollateral<'info> {
    #[account(mut)]
    pub user_token_account: Account<'info,TokenAccount>,
    pub mint: Account<'info,Mint>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info,System>
}