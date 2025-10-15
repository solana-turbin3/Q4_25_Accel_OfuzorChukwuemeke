use anchor_lang::{prelude::*, system_program::Transfer};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{Escrow, ESCROW_SEED};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        init,
        payer = maker,
        space = Escrow::DISCRIMINATOR.len() + Escrow::INIT_SPACE,
        seeds = [ESCROW_SEED, maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl Make<'_> {
    pub fn handler(
        ctx: Context<Make>,
        seed: u64,
        deposit_amount: u64,
        receive_amount: u64,
        unlock_time: i64,
    ) -> Result<()> {
        let Make {
            escrow,
            maker,
            maker_ata_a,
            mint_a,
            mint_b,
            token_program,
            vault,
            ..
        } = ctx.accounts;
        escrow.set_inner(Escrow {
            bump: ctx.bumps.escrow,
            seed,
            receive_amount,
            unlock_time,
            maker: maker.key(),
            mint_a: mint_a.key(),
            mint_b: mint_b.key(),
        });

        transfer_checked(CpiContext::new(
            token_program.to_account_info(),
            TransferChecked {
                authority:maker.to_account_info(),
                from:maker_ata_a.to_account_info(),
                mint:mint_a.to_account_info(),
                to:vault.to_account_info(),
            }
        ),
        deposit_amount,
        mint_a.decimals,
    )
    }
}
