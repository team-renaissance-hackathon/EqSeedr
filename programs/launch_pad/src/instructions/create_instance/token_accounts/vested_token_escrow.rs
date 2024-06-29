use crate::states::{ProgramAuthority, Session};

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct CreateVestedTokenEscrow<'info> {
    // session authority / creator
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [b"authority"],
        bump = program_authority.bump
    )]
    pub program_authority: Account<'info, ProgramAuthority>,

    #[account(
        init,
        payer = authority,
        seeds = [
            session.key().as_ref(),
            b"vested-token-escrow",
        ],
        bump,
        token::mint = token_mint,
        token::authority = program_authority,
        token::token_program = token_program,
    )]
    pub new_vested_token_escrow: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        has_one = authority
    )]
    pub session: Account<'info, Session>,

    pub token_mint: InterfaceAccount<'info, Mint>,
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
