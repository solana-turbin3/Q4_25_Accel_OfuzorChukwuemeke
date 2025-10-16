use anchor_lang::prelude::*;
pub mod constants;
pub mod instructions;

pub mod error;
pub use error::*;

pub mod tests;

pub use constants::*;
pub mod state;

pub use instructions::*;
pub use state::*;
declare_id!("xwTU4rUFVXiucJwsDKNvxqKnCMmvkuBdac4XsnF64X8");

#[program]
pub mod litesvm_escrows {
    use super::*;

    pub fn make(
        ctx: Context<Make>,
        seed: u64,
        deposit_amount: u64,
        receive_amount: u64,
        unlock_time: i64,
    ) -> Result<()> {
        Make::handler(ctx, seed, deposit_amount, receive_amount, unlock_time)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
