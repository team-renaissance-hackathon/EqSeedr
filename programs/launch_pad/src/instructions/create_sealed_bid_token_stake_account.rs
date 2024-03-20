use super::super::states::Session;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct CreateSealedBidTokenStakeAccount<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        has_one = authority
    )]
    pub session: Account<'info, Session>,

    #[account(
        init,
        payer =  authority,
        associated_token::authority = session,
        associated_token::mint = stake_token_mint,
        associated_token::token_program = token_program,
    )]
    pub new_sealed_bid_token_stake_account: Account<'info, TokenAccount>,

    #[account(
        constraint = !session.is_valid_token_mint(session_token_mint.key()),
    )]
    pub session_token_mint: Account<'info, Mint>,

    pub stake_token_mint: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateSealedBidTokenStakeAccount>) -> Result<()> {
    // emit log
    // new token account
    // type of token account
    // session id
    msg!("{}", ctx.accounts.new_sealed_bid_token_stake_account.key());
    Ok(())
}
