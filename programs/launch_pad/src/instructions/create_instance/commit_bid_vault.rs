use crate::states::{ProgramAuthority, Session};
use crate::utils::errors::ErrorCode;

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct CreateCommitBidVault<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        constraint = program_authority.is_initialized == true,
        seeds = [b"authority"],
        bump = program_authority.bump
    )]
    pub program_authority: Account<'info, ProgramAuthority>,

    #[account(
        constraint = !session.has_valid_commit_bid_vault 
            @ ErrorCode::CommitBidVaultAlreadyExist,
        init,
        payer = authority,
        seeds = [
            session.key().as_ref(),
            b"commit-bid-vault",
        ],
        bump,
        token::mint = bid_token_mint,
        token::authority = program_authority,
        token::token_program = token_program,
    )]
    pub new_commit_bid_vault: InterfaceAccount<'info, TokenAccount>,

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

pub fn handler(ctx: Context<CreateCommitBidVault>) -> Result<()> {
    // emit log
    // new token account
    // type of token account
    msg!("{}", ctx.accounts.new_commit_bid_vault.key());
    Ok(())
}

// TODO!
// - account inits need to reflect anchor 0.30.0
// - need to implement event logs.
