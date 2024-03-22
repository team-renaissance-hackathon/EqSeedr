use super::super::{
    states::{ProgramAuthority, SealedBidByIndex, SealedBidRound, Session},
    utils::*,
};
use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct SubmitSealedBid<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        // space = SealedBidByIndex::LEN,
        space = 10000,
        seeds = [
            sealed_bid_round.next_index().as_ref(),
            session.key().as_ref(),
            b"sealed-bid-by-index",
        ],
        bump
    )]
    pub new_sealed_bid_by_index: Account<'info, SealedBidByIndex>,

    #[account(
        mut,
        constraint = sealed_bid_round.session == session.key(),
    )]
    pub sealed_bid_round: Account<'info, SealedBidRound>,

    #[account(
        mut,
        constraint = bidder_token_account.owner == authority.key()
    )]
    pub bidder_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        // right now this constraint wont work, no staking account is stored.
        // constraint = session.is_valid_staking_account(session_stake_token_account.key())
    )]
    pub session_stake_token_account: Account<'info, TokenAccount>,

    #[account(
        // right now this constraint wont work since I have to create a cpi so the program authority can be
        // the mint authority.
        // constraint = token_mint.mint_authority.unwrap() == program_authority.key(),
    )]
    pub token_mint: Account<'info, Mint>,

    pub program_authority: Account<'info, ProgramAuthority>,
    pub session: Account<'info, Session>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SubmitSealedBid>, commit_hash: Pubkey) -> Result<()> {
    let SubmitSealedBid {
        authority,
        new_sealed_bid_by_index,
        sealed_bid_round,
        session,
        bidder_token_account,
        session_stake_token_account,
        token_program,
        ..
    } = ctx.accounts;

    new_sealed_bid_by_index.initialize(
        ctx.bumps.new_sealed_bid_by_index,
        sealed_bid_round.get_index(),
        session.key(),
        authority.key(),
        session.staking_amount,
        commit_hash,
    );

    sealed_bid_round.update_total_sealed_bids();

    transfer(
        CpiContext::new(
            token_program.to_account_info(),
            Transfer {
                from: bidder_token_account.to_account_info(),
                to: session_stake_token_account.to_account_info(),
                authority: authority.to_account_info(),
            },
        ),
        session.staking_amount,
    )?;

    Ok(())
}
