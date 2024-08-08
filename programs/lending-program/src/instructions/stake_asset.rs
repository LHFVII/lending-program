use anchor_lang::prelude::*;

use crate::error::LendingProgramError;
use crate::instructions::initialize_user::UserAccount;


pub fn stake_asset(
    ctx: Context<StakeAsset>,
    amount: u64
    ) -> Result<()>{
        Ok(())
    }

    #[derive(Accounts)]
    pub struct StakeAsset<'info> {
        pub signer: Signer<'info>,

    }