use super::super::states::ProgramAuthority;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct CreateCommitTokenAccount<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        // constraint = program_authority.is_initialized == true,
        seeds = [b"authority"],
        bump = program_authority.bump
    )]
    pub program_authority: Account<'info, ProgramAuthority>,

    #[account(
        init,
        payer = authority,
        associated_token::mint = bid_token_mint,
        associated_token::authority = program_authority,
        associated_token::token_program = token_program,
    )]
    pub new_commit_token_account: Account<'info, TokenAccount>,

    pub bid_token_mint: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateCommitTokenAccount>) -> Result<()> {
    // emit log
    // new token account
    // type of token account
    msg!("{}", ctx.accounts.new_commit_token_account.key());
    Ok(())
}
