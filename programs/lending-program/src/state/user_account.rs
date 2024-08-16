use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserAccount{
    pub owner: Pubkey,
    pub allowed_borrow_amount_in_usdc: u64,
    pub borrowed_amount_in_usdc: u64,
    pub stake_points: u32,
    pub amount_staked: u8,
    pub bump: u8,
}