use anchor_lang::prelude::*;

pub fn borrow_asset(
    ctx: Context<BorrowAsset>,
    ) -> Result<()>{
        
    Ok(())
}

#[derive(Accounts)]
pub struct BorrowAsset<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info,System>
}