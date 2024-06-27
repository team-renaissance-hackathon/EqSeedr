use anchor_lang::prelude::*;

use crate::states::ProgramAuthority;

use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    // needs multi sig functionality
    #[account(mut)]
    pub signer: Signer<'info>,

    pub program_authority: Account<'info, ProgramAuthority>,

    #[account(mut)]
    pub program_token_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub receipent: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [
            program_authority.key().clone().as_ref(),
            b"eqseedr-token-mint",
        ],
        bump,
    )]
    pub token_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handler(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
    let TransferTokens {
        token_program,
        program_token_vault,
        token_mint,
        program_authority,
        receipent,
        ..
    } = ctx.accounts;
    // needs to be an approved amount from a contract
    // needs to be an approved dest from a contract

    let seeds = &[b"authority", &[program_authority.bump][..]];
    let signer_seeds = &[&seeds[..]];

    transfer_checked(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            TransferChecked {
                from: program_token_vault.to_account_info(),
                to: receipent.to_account_info(),
                authority: program_authority.to_account_info(),
                mint: token_mint.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
        token_mint.decimals,
    )?;

    Ok(())
}
