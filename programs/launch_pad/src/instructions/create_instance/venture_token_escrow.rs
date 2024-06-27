use crate::states::{ProgramAuthority, Session};
use crate::utils::errors::ErrorCode;

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct CreateVentureTokenEscrow<'info> {
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
            b"venture-token-escrow",
        ],
        bump,
        token::mint = bid_token_mint,
        token::authority = program_authority,
        token::token_program = token_program,
    )]
    pub new_venture_token_escrow: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        has_one = authority,
    )]
    pub session: Account<'info, Session>,

    #[account(
        constraint = program_authority.is_valid_token(bid_token_mint.key())
            @ ErrorCode::InvalidBidToken,
    )]
    pub bid_token_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateVentureTokenEscrow>) -> Result<()> {
    msg!("{}", ctx.accounts.new_venture_token_escrow.key());
    Ok(())
}
