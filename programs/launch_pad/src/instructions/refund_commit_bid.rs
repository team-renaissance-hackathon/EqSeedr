use crate::states::{
    // STATE ACCOUNTS
    CommitLeaderBoard,
    CommitQueue,
    ProgramAuthority,
    SealedBidByIndex,
    SealedBidRound,
    Session,
};

use crate::utils::errors::ErrorCode;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

#[derive(Accounts)]
pub struct RefundCommitBidBySession<'info> {
    // investor
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        constraint = !sealed_bid_by_index.is_commit
            @ ErrorCode::BidAlreadyCommited,
        constraint = sealed_bid_by_index.owner == authority.key()
            @ ErrorCode::InvalidOwnerOfSealedBidByIndex,
    )]
    pub sealed_bid_by_index: Account<'info, SealedBidByIndex>,

    #[account(
        mut,
        constraint = !sealed_bid_round.is_valid_session(session.key())
        // currently can't test right now
        // constraint = !sealed_bid_round.is_valid_unsealed_bid_phase(),
    )]
    pub sealed_bid_round: Account<'info, SealedBidRound>,

    #[account(
        mut,
        constraint = !commit_leader_board.is_valid_session(session.key()),
        // this also doesn't work
        // constraint = !commit_leader_board.is_valid_indexed_commit_bid(&sealed_bid_by_index)
    )]
    pub commit_leader_board: Account<'info, CommitLeaderBoard>,

    #[account(
        mut,
        constraint = !commit_queue.is_valid_session(session.key()),
        // the bug seems to exist in this validation
        // only happens win the bid_index is the last bid_index
        constraint = !commit_queue.is_valid_insert(&commit_leader_board, &sealed_bid_by_index)
    )]
    pub commit_queue: Account<'info, CommitQueue>,

    #[account(
        mut,
        constraint = bidder_token_account.owner == authority.key()
    )]
    pub bidder_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        // constraint = session_commit_token_account.owner == session.key()
        constraint = session_commit_token_account.owner == program_authority.key()

    )]
    pub session_commit_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        // constraint = program_authority.is_valid_token(token_mint.key())
    )]
    pub token_mint: InterfaceAccount<'info, Mint>,

    pub program_authority: Account<'info, ProgramAuthority>,
    pub session: Account<'info, Session>,
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handler(ctx: Context<RefundCommitBidBySession>) -> Result<()> {
    let RefundCommitBidBySession {
        // authority,
        sealed_bid_by_index,
        // commit_leader_board,
        // commit_queue,
        session,
        bidder_token_account,
        session_commit_token_account,
        token_program,
        token_mint,
        program_authority,
        ..
    } = ctx.accounts;

    // Validate that the bid is actually committed
    require!(sealed_bid_by_index.is_commit, ErrorCode::BidNotCommitted);

    // Validate that the bid isn't already refunded
    require!(
        !sealed_bid_by_index.is_refunded,
        ErrorCode::BidIsAlreadyRefunded
    );
    require!(
        !sealed_bid_by_index.is_refunded,
        ErrorCode::BidIsAlreadyRefunded
    );

    // don't need to remove aynthing from commit leaderboard,
    // as refund only happens at the end of the unsealed bid phase(?)
    //let node = commit_leader_board.get_node(sealed_bid_by_index.commit_leader_board_index);

    sealed_bid_by_index.refunded();

    // Construct the program authority signer

    let seeds = &[b"auhtority", &[program_authority.bump][..]];
    let seeds = &[b"authority", &[program_authority.bump]];
    let signer_seeds = &[&seeds[..]];

    transfer_checked(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            TransferChecked {
                from: session_commit_token_account.to_account_info(),
                to: bidder_token_account.to_account_info(),
                authority: program_authority.to_account_info(),
                mint: token_mint.to_account_info(),
            },
            signer_seeds,
        ),
            signer_seeds,
        ),
        session.staking_amount,
        token_mint.decimals,
    )?;

    Ok(())
}

//  refund commit bid
//      - data
//          - sealed_bid_by_index.commit_leader_board_index
//          - sealed_bid_by_index.index
//          - sealed_bid_by_index.owner?
//          - sealed_bid_by_index.is_unsealed
//          - commit_leader_board.amount
//      - validate
//          - commit_leader_board.index == sealed_bid_by_index.index
//          - commit_leader_board.session == session
//          - sealed_bid_by_index.session == session
//          - commit_queue.session == session
//          - commit_queue.is_valid_insert
//          - sealed_bid_round.session == session
//          - token_mint.is_valid_bid_token_mint
//          - sealed_bid_by_index.is_commit == true
//          - sealed_bid_by_index.is_refunded == false

