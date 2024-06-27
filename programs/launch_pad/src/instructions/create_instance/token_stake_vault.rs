use crate::states::{ProgramAuthority, Session};
use crate::utils::errors::ErrorCode;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct CreateTokenStakeVault<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        has_one = authority
    )]
    pub session: Account<'info, Session>,

    #[account(
        seeds = [
            b"authority",
        ],
        bump = program_authority.bump,
    )]
    pub program_authority: Account<'info, ProgramAuthority>,

    #[account(
        init,
        payer = authority,
        seeds = [
            session.key().as_ref(),
            token_stake_mint.key().as_ref(),
            b"token-stake-vault"
        ],
        bump,
        token::authority = program_authority,
        token::mint = token_stake_mint,
        token::token_program = token_program,
    )]
    pub new_token_stake_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        constraint = !session.is_valid_token_mint(venture_token_mint.key())
            @ ErrorCode::InvalidVentureTokenMint,
    )]
    pub venture_token_mint: InterfaceAccount<'info, Mint>,

    pub token_stake_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateTokenStakeVault>) -> Result<()> {
    msg!(
        "New Token Stake Vault: {}",
        ctx.accounts.new_token_stake_vault.key()
    );
    Ok(())
}
