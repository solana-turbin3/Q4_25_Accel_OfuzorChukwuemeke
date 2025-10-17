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
        deposit: u64,
        receive: u64,
        open_in: i64,
    ) -> Result<()> {
        ctx.accounts
            .init_escrow(seed, receive, open_in, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)
    }

     pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw_and_close_vault()
    }

    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        ctx.accounts.cancel_and_close_vault()
    }
}
