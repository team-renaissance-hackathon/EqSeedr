use crate::states::{SealedBidRound, Session};
use crate::utils::errors::ProgramError;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateSessionSealedBidRound<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = SealedBidRound::LEN,
        seeds = [
            session.key().as_ref(),
            b"sealed-bid-round",
        ],
        bump
    )]
    pub new_sealed_bid_round: Account<'info, SealedBidRound>,

    #[account(
        mut,
        has_one = authority,
        constraint = !session.has_sealed_bid_round @ ProgramError::SessionSealedBidRoundAlreadyExist
    )]
    pub session: Account<'info, Session>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateSessionSealedBidRound>) -> Result<()> {
    let CreateSessionSealedBidRound {
        authority,
        new_sealed_bid_round,
        session,
        ..
    } = ctx.accounts;

    new_sealed_bid_round.initialize(
        ctx.bumps.new_sealed_bid_round,
        // I am not sure if I need tc clone. will test it later
        authority.key().clone(),
        session.key().clone(),
    );

    session.add_sealed_bid_round();

    Ok(())
}
