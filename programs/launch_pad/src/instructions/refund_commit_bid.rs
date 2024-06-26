use crate::states::{
    // STATE ACCOUNTS
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
    pub bidder: Signer<'info>,

    #[account(mut)]
    pub sealed_bid_by_index: Account<'info, SealedBidByIndex>,

    #[account(
        mut,
        constraint = !sealed_bid_round.is_valid_session(session.key())
            @ ErrorCode::InvalidSession,
    )]
    pub sealed_bid_round: Account<'info, SealedBidRound>,

    #[account(
        mut,
        constraint = !commit_queue.is_valid_session(session.key())
            @ ErrorCode::InvalidSession,
    )]
    pub commit_queue: Account<'info, CommitQueue>,

    #[account(
        mut,
        constraint = bidder_token_account.owner == bidder.key()
    )]
    pub bidder_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [
            session.key().as_ref(),
            b"commit-bid-vault",
        ],
        bump,
    )]
    pub commit_bid_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        constraint = program_authority.is_valid_token(token_mint.key())
    )]
    pub token_mint: InterfaceAccount<'info, Mint>,

    pub program_authority: Account<'info, ProgramAuthority>,
    pub session: Account<'info, Session>,
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handler(ctx: Context<RefundCommitBidBySession>) -> Result<()> {
    let RefundCommitBidBySession {
        sealed_bid_by_index,
        bidder_token_account,
        commit_bid_vault,
        token_program,
        token_mint,
        program_authority,
        ..
    } = ctx.accounts;

    // Validate that the bid is actually committed
    require!(sealed_bid_by_index.is_commit, ErrorCode::BidNotCommitted);

    // Validate that the bid isn't already refunded
    require!(
        !sealed_bid_by_index.is_bid_refunded,
        ErrorCode::BidIsAlreadyRefunded
    );

    sealed_bid_by_index.bid_refunded();

    // Construct the program authority signer
    let seeds = &[b"authority", &[program_authority.bump][..]];
    let signer_seeds = &[&seeds[..]];

    transfer_checked(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            TransferChecked {
                from: commit_bid_vault.to_account_info(),
                to: bidder_token_account.to_account_info(),
                authority: program_authority.to_account_info(),
                mint: token_mint.to_account_info(),
            },
            signer_seeds,
        ),
        sealed_bid_by_index.bid_amount,
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
