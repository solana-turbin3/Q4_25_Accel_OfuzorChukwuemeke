use anchor_lang::prelude::*;
mod instructions;
mod state;

use instructions::*;
use spl_discriminator::SplDiscriminate;
use spl_tlv_account_resolution::state::ExtraAccountMetaList;
use spl_transfer_hook_interface::instruction::{
    ExecuteInstruction, InitializeExtraAccountMetaListInstruction,
};

declare_id!("7imqHRHzPhbJ3DoYHqLpkjF7bZnAXe9NbtCYHcgZbzv9");

#[program]
pub mod whitelist_transferhook {
    use super::*;

    pub fn initialize_whitelist(ctx: Context<InitializeWhitelist>) -> Result<()> {
        ctx.accounts.initialize_whitelist(ctx.bumps)
    }

    pub fn add_to_whitelist(ctx: Context<WhitelistOperations>) -> Result<()> {
        ctx.accounts.add_to_whitelist()
    }

    pub fn initialize_mint_with_hook(ctx: Context<InitializeMintWithHook>) -> Result<()> {
        ctx.accounts.initialize()
    }

    #[instruction(discriminator = InitializeExtraAccountMetaListInstruction::SPL_DISCRIMINATOR_SLICE)]
    pub fn initialize_transfer_hook(ctx: Context<InitializeExtraAccountMetaList>) -> Result<()> {
        msg!("Initializing Transfer Hook...");

        let extra_account_metas =
            InitializeExtraAccountMetaList::extra_account_metas(&ctx.accounts.mint.key())?;

        msg!("Extra Account Metas: {:?}", extra_account_metas);
        msg!("Extra Account Metas Length: {}", extra_account_metas.len());

        ExtraAccountMetaList::init::<ExecuteInstruction>(
            &mut ctx.accounts.extra_account_meta_list.try_borrow_mut_data()?,
            &extra_account_metas,
        )?;

        Ok(())
    }

    #[instruction(discriminator = ExecuteInstruction::SPL_DISCRIMINATOR_SLICE)]
    pub fn transfer_hook(ctx: Context<TransferHook>, amount: u64) -> Result<()> {
        // Call the transfer hook logic
        ctx.accounts.transfer_hook(amount)
    }
}
