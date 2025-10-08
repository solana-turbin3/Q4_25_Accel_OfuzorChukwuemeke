use anchor_lang::prelude::*;

declare_id!("7imqHRHzPhbJ3DoYHqLpkjF7bZnAXe9NbtCYHcgZbzv9");

#[program]
pub mod whitelist_transferhook {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
