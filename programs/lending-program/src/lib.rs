pub mod instructions;
pub mod error;

use anchor_lang::prelude::*;

use instructions::*;

declare_id!("77B3AdNp6RRzsVMQSWAUVv28RXmA8YJVAfWkimRTwXi6");

#[program]
pub mod lending_program {
    use super::*;

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        instructions::initialize_user(ctx)
    }
}


