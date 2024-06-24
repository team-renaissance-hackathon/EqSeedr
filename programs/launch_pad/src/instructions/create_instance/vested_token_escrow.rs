use crate::states::Session;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

#[derive(Accounts)]
pub struct CreateVestedTokenEscrow<'info> {
    // session authority / creator
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [
            token_mint.key().as_ref(),
            b"escrow",
        ],
        bump,
    )]
    pub escrow_authority: SystemAccount<'info>,

    #[account(
        init,
        payer = authority,
        associated_token::mint = token_mint,
        associated_token::authority = escrow_authority,
        associated_token::token_program = token_program,
    )]
    pub new_vested_token_escrow: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        has_one = authority
    )]
    pub session: Account<'info, Session>,

    pub token_mint: InterfaceAccount<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateVestedTokenEscrow>) -> Result<()> {
    let CreateVestedTokenEscrow {
        new_vested_token_escrow,
        token_mint,
        ..
    } = ctx.accounts;

    msg!(
        "created new vested token escrow {}, token mint {}",
        // inputs
        new_vested_token_escrow.key(),
        token_mint.key(),
    );

    Ok(())
}
