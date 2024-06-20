use crate::states::ProgramAuthority;

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
        constraint = program_authority.is_initialized == true,
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

// THOUGHTS:
// : if there is going to be more than one token that will be used to make bids, then there
// will need to be multiple commit token accounts.
// : the argument for using allowing to have multiple stable coins for user options to use for bids,
// is availabitlity and liquidity. what if at a given point there is not
// 1 million USDC at that moment available for users to place there bids.
// I doubt such a situation would happen but there is a possibility.
// having multiple stable coins as an option provides more liquidity and reduces that risk.
// FINAL DECISION:
// the commit token account will be ephemeral, instance based to a specific active session.
// and it will be created only with USDC token mint. if in the future, there needs to be
// used with optional token mints of other stable coins then that change can be made in the future.

// TODO!
// - Needs update to interface with all SPL token standards and extensions.
// - account inits need to reflect anchor 0.30.0
// - need to implement event logs.
// - need to update the session state account to tie the commit token acccount with the session account.
