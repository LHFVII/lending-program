use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct User{
    pub owner: Pubkey,
    
    pub stake_points: u32,
    pub amount_staked: u8,
    
    pub deposited_usdc: u64,
    pub deposited_sol: u64,

    pub deposited_usdc_shares: u64, 
    
    pub health_factor: u64,
    
    pub last_updated: i64,
    pub bump: u8,
}