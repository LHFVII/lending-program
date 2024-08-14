use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserDepositAccount{
    pub amount: u64,
}