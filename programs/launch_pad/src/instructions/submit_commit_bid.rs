use crate::states::{
    CommitLeaderBoard, CommitQueue, ProgramAuthority, SealedBidByIndex, SealedBidRound, Session,
};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

#[derive(Accounts)]
pub struct CommitBidBySession<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        // constraint = !sealed_bid_by_index.is_commit,
        // @ BidAlreadyCommited
        // constraint = sealed_bid_by_index.owner == authority.key(),
        // @ InvalidOwnerOfSealedBidByIndex
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
        // constraint = !commit_leader_board.is_valid_session(session.key()),
        // this also doesn't work
        // constraint = !commit_leader_board.is_valid_indexed_commit_bid(&sealed_bid_by_index)
    )]
    pub commit_leader_board: Account<'info, CommitLeaderBoard>,

    #[account(
        mut,
        // constraint = !commit_queue.is_valid_session(session.key()),
        // the bug seems to exist in this validation
        // only happens win the bid_index is the last bid_index
        // constraint = !commit_queue.is_valid_insert(&commit_leader_board, &sealed_bid_by_index)
    )]
    pub commit_queue: Account<'info, CommitQueue>,

    #[account(
        mut,
        // constraint = bidder_token_account.owner == authority.key()
    )]
    pub bidder_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        // constraint = commit_bid_vault.owner == session.key()
        constraint = commit_bid_vault.owner == program_authority.key()

    )]
    pub commit_bid_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        // constraint = program_authority.is_valid_token(token_mint.key())
    )]
    pub token_mint: InterfaceAccount<'info, Mint>,

    pub program_authority: Account<'info, ProgramAuthority>,
    pub session: Account<'info, Session>,
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handler(ctx: Context<CommitBidBySession>) -> Result<()> {
    let CommitBidBySession {
        authority,
        sealed_bid_by_index,
        commit_leader_board,
        commit_queue,
        bidder_token_account,
        commit_bid_vault,
        token_program,
        token_mint,
        ..
    } = ctx.accounts;

    let node = commit_leader_board.get_node(sealed_bid_by_index.commit_leader_board_index);
    commit_queue.insert(node, &sealed_bid_by_index);

    sealed_bid_by_index.add_commit();

    transfer_checked(
        CpiContext::new(
            token_program.to_account_info(),
            TransferChecked {
                from: bidder_token_account.to_account_info(),
                to: commit_bid_vault.to_account_info(),
                authority: authority.to_account_info(),
                mint: token_mint.to_account_info(),
            },
        ),
        sealed_bid_by_index.bid_amount,
        token_mint.decimals,
    )?;

    commit_queue.remove();

    Ok(())
}

//  submit commit bid
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
//      - update
//          - STATES:
//              - queue is empty
//                  - add into index 0 -> {owner, amount, owner_index}
//                  - log new insert and position
//                  - transfer new insert funds into commit queue fund account
//              - queue not empty
//                  - itereate until valid position insert
//                  - log new insert and position
//                  - transfer new insert funds into commit queue fund account
//              - queue is filled
//                  - itereate unti valid postion insert
//                  - remove last element
//                  - log new insert and position, log removed element
//                  - transfer new insert funds into commit queue fund account
//                  - transfer removed invester from commit queue fund account to investor account
//                  - record / update last index investor into commit leader board as the cut off point
//                      where any account above that point that hasn't commited to bid will lose their staked amount

// TODO!
// - need to implement event logs
// - add / update validations with correct and working errors, need to explore why the errors are not working
// - implement the refund instruction in seperate file

// refund of commit:
// handle refund commit transfer here?
// or handle refund commit transfter in a seperate transaction?
// in effect becoming a pull transfer
// if handling refund commit here, if transactions are happening fast
// could be an issue becuase the acount to refund back
// has already been refunded and the next transaction could be
// wrong account to refund back.
// I think this can be handled in a look up table
// to dynamically pull accounts that could be needed
// but would need to explore how to use it. or even see
// if its viable
// so for now it will be in a seperate instruction
// updated: this process will be through the investor that made
// the bid to refund their commit bid
