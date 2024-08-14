use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct PoolConfig{
    pub max_amount: u64
}