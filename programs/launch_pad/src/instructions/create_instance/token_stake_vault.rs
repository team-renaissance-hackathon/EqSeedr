use crate::states::Session;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

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
            session.key().as_ref(),
            stake_token_mint.key().as_ref(),
            b"stake-authority",
        ],
        bump
    )]
    pub stake_authority: SystemAccount<'info>,

    #[account(
        init,
        payer = authority,
        associated_token::authority = stake_authority,
        associated_token::mint = stake_token_mint,
        associated_token::token_program = token_program,
    )]
    pub new_token_stake_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        constraint = !session.is_valid_token_mint(session_token_mint.key()),
    )]
    pub session_token_mint: InterfaceAccount<'info, Mint>,

    pub stake_token_mint: InterfaceAccount<'info, Mint>,

    pub associated_token_program: Program<'info, AssociatedToken>,
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
