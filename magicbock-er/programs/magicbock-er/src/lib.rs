#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::anchor::ephemeral;

mod state;
mod instructions;

use instructions::*;
use anchor_lang::prelude::*;


declare_id!("DhFe9eBJzHh7zSWfVMrFbbrxwGpif8c4WvfbdTVjVf5e");

#[program]
pub mod magicbock_er {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
