use crate::states::{CommitLeaderBoard, SealedBidByIndex, SealedBidRound, Session};
use crate::utils::errors::ErrorCode;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(amount: u64, index: u32, secret: [u8; 32])]
pub struct SubmitUnsealedBid<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        constraint = !sealed_bid_by_index.is_valid_unsealed_bid(amount, secret)
            @ ErrorCode::InvalidUnsealedBid,
    )]
    pub sealed_bid_by_index: Account<'info, SealedBidByIndex>,

    #[account(
        mut,
        // constraint = !sealed_bid_round.is_valid_session(session.key()),

        // can't test this validation yet.
        // constraint = !sealed_bid_round.is_valid_unsealed_bid_phase(),
        // constraint = !sealed_bid_round.is_valid_unsealed_bid(),
    )]
    pub sealed_bid_round: Account<'info, SealedBidRound>,

    #[account(
        mut,
        // constraint = !commit_leader_board.is_valid_session(session.key()),
        // constraint = !commit_leader_board.is_valid_node(index, amount)
    )]
    pub commit_leader_board: Account<'info, CommitLeaderBoard>,

    pub session: Account<'info, Session>,
}

pub fn handler(ctx: Context<SubmitUnsealedBid>, amount: u64, index: u32) -> Result<()> {
    let SubmitUnsealedBid {
        sealed_bid_by_index,
        commit_leader_board,
        ..
    } = ctx.accounts;

    sealed_bid_by_index.unsealed_bid(commit_leader_board.pool.total, amount);

    let node = commit_leader_board.create_node(
        sealed_bid_by_index.bid_index,
        sealed_bid_by_index.bid_amount,
    );

    commit_leader_board.add(&mut node.clone(), index);

    Ok(())
}

// TODO!
// - need to implement event logs
// - add / update validations with correct and working errors, need to explore why the errors are not working
// - refactor the commit leader board
// - - into generic linked list
// - - zero copy account structure -> though the data set maybe small enough to not need a zero copy
//     but may need go the account to the max heap size.
