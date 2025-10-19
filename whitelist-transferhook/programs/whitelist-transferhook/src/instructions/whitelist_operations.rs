use anchor_lang::{prelude::*};

use crate::state::whitelist::Whitelist;

#[derive(Accounts)]
pub struct WhitelistOperations<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    /// The user to whitelist or remove
    pub user: Signer<'info>,
    #[account(
        init_if_needed,
        payer = admin,
        seeds = [b"whitelist", mint.key().as_ref(), user.key().as_ref()],
        bump,
        space = 8 + 32 + 32 + 1 // discriminator + user + mint + bump
    )]
    pub whitelist: Account<'info, Whitelist>,

    /// CHECK: The mint that this whitelist applies to
    pub mint: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RemoveWhitelist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    /// The user to remove
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"whitelist", mint.key().as_ref(), user.key().as_ref()],
        bump,
        close = admin
    )]
    pub whitelist: Account<'info, Whitelist>,

    /// CHECK: The mint that this whitelist applies to
    pub mint: UncheckedAccount<'info>,
}

impl<'info> WhitelistOperations<'info> {
    pub fn add_to_whitelist(&mut self) -> Result<()> {
        self.whitelist.set_inner(crate::state::Whitelist {
            user: self.user.key(),
            mint: self.mint.key(),
            bump: self.whitelist.bump,
        });

        Ok(())
    }
}

impl<'info> RemoveWhitelist<'info> {
    pub fn remove_from_whitelist(&mut self) -> Result<()> {
        Ok(())
    }
}
