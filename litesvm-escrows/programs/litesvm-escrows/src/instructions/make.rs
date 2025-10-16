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

        transfer_checked(
            CpiContext::new(
                token_program.to_account_info(),
                TransferChecked {
                    authority: maker.to_account_info(),
                    from: maker_ata_a.to_account_info(),
                    mint: mint_a.to_account_info(),
                    to: vault.to_account_info(),
                },
            ),
            deposit_amount,
            mint_a.decimals,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::constants::{
        ASSOCIATED_TOKEN_PROGRAM_ID, MINT_DECIMALS, PROGRAM_ID, SYSTEM_PROGRAM_ID, TOKEN_PROGRAM_ID,
    };
    use crate::tests::pda::get_escrow_pda;
    use crate::tests::utils::{
        build_and_send_transaction, fetch_account, init_ata, init_mint, setup,
    };
    use crate::Escrow;
    use crate::{accounts::Make as MakeAccounts, instruction::Make as MakeData};
    use anchor_lang::{prelude::instruction::Instruction, InstructionData, ToAccountMetas};
    use anchor_spl::token::{Token, TokenAccount};
    use solana_keypair::Keypair;
    use solana_program::native_token::LAMPORTS_PER_SOL;
    use solana_signer::Signer;
    use spl_associated_token_account::get_associated_token_address;

    #[test]
    fn make_an_escrow() {
        let (litesvm, _default_payer) = &mut setup();

        let maker = Keypair::new();
        litesvm.airdrop(&maker.pubkey(), LAMPORTS_PER_SOL).unwrap();

        let mint_a = init_mint(litesvm, TOKEN_PROGRAM_ID, MINT_DECIMALS, 100);
        let mint_b = init_mint(litesvm, TOKEN_PROGRAM_ID, MINT_DECIMALS, 100);

        let maker_ata_a = init_ata(litesvm, mint_a, maker.pubkey(), 10);

        let pre_maker_ata_a_bal = fetch_account::<TokenAccount>(litesvm, &maker_ata_a).amount;

        let deposit_amount = 1;
        let receive_amount = 2;
        let seed = 1234u64;
        let unlock_time = 60 * 60 * 24; //  1 day from now

        let escrow = get_escrow_pda(maker.pubkey(), seed);
        let vault_ata_address = get_associated_token_address(&escrow, &mint_a);

        let ix = Instruction {
            accounts: MakeAccounts {
                associated_token_program: ASSOCIATED_TOKEN_PROGRAM_ID,
                escrow,
                maker: maker.pubkey(),
                maker_ata_a,
                mint_a,
                mint_b,
                system_program: SYSTEM_PROGRAM_ID,
                token_program: TOKEN_PROGRAM_ID,
                vault: vault_ata_address,
            }
            .to_account_metas(None),
            data: MakeData {
                deposit_amount,
                receive_amount,
                seed,
                unlock_time,
            }
            .data(),
            program_id: PROGRAM_ID,
        };
        let _ = build_and_send_transaction(litesvm, &[&maker], &maker.pubkey(), &[ix]);

        let escrow_data = fetch_account::<Escrow>(litesvm, &escrow);

        assert_eq!(escrow_data.seed, seed);
        assert_eq!(escrow_data.receive_amount, receive_amount);
        assert_eq!(escrow_data.maker, maker.pubkey());
        assert_eq!(escrow_data.mint_a, mint_a);
        assert_eq!(escrow_data.mint_b, mint_b);

        let vault_ata_bal = fetch_account::<TokenAccount>(litesvm, &vault_ata_address).amount;

        assert_eq!(vault_ata_bal, deposit_amount);

        let post_maker_ata_a_bal = fetch_account::<TokenAccount>(litesvm, &maker_ata_a).amount;

        assert_eq!(pre_maker_ata_a_bal,post_maker_ata_a_bal + deposit_amount);
    }
}
