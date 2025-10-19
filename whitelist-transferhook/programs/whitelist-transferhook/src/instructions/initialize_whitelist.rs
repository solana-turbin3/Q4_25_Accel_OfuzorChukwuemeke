use anchor_lang::prelude::*;

use crate::state::Whitelist;

#[derive(Accounts)]
pub struct InitializeWhitelist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    /// The user to whitelist
    /// CHECK: Not dangerous, just a Pubkey passed in
    pub user: UncheckedAccount<'info>,
    /// CHECK: The mint that this whitelist applies to
    pub mint: UncheckedAccount<'info>,

    #[account(
        init,
        payer = admin,
        space = 8 + 32 + 32 + 1, // discriminator + user + mint + bump
        seeds = [b"whitelist", mint.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub whitelist: Account<'info, Whitelist>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeWhitelist<'info> {
    pub fn initialize_whitelist(&mut self, bumps: InitializeWhitelistBumps) -> Result<()> {
        self.whitelist.set_inner(Whitelist {
            user: self.user.key(),
            mint: self.mint.key(),
            bump: bumps.whitelist,
        });

        Ok(())
    }
}
