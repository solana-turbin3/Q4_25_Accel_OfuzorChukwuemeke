use anchor_lang::prelude::*;
pub mod instructions;
pub mod constants;

pub mod error;
pub use error::*;

pub mod tests;

pub use constants::*;
pub mod state;

pub use state::*;
pub use instructions::*;
declare_id!("xwTU4rUFVXiucJwsDKNvxqKnCMmvkuBdac4XsnF64X8");

#[program]
pub mod litesvm_escrows {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

