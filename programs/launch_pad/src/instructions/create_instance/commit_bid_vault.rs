use crate::states::{ProgramAuthority, Session};
use crate::utils::errors::ErrorCode;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

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
        seeds = [
            session.key().as_ref(),
            program_authority.key().as_ref(),
            b"vault",
        ],
        bump,
    )]
    pub vault_authority: SystemAccount<'info>,

    #[account(
        init,
        payer = authority,
        associated_token::mint = bid_token_mint,
        associated_token::authority = vault_authority,
        associated_token::token_program = token_program,
    )]
    pub new_commit_bid_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        has_one = authority,
        constraint = !session.has_valid_commit_bid_vault 
            @ ErrorCode::CommitBidVaultAlreadyExist,
    )]
    pub session: Account<'info, Session>,

    #[account(
        constraint = program_authority.is_valid_token(bid_token_mint.key())
            @ ErrorCode::InvalidBidToken,
    )]
    pub bid_token_mint: InterfaceAccount<'info, Mint>,

    pub associated_token_program: Program<'info, AssociatedToken>,
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
