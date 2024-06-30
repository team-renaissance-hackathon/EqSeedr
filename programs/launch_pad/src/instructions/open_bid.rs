use crate::states::{
    // STATE ACCOUNTS
    CommitQueue,
    ProgramAuthority,
    Session,
    // SessionStatus,
    TickBidRound,
    VestedAccountByIndex,
    VestedAccountByOwner,
    VestedConfig,
};
use crate::utils::errors::ErrorCode;

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

#[derive(Accounts)]
pub struct OpenBid<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    // OPTION:
    // verified Account to execute open bid
    // state account of list of verfied accounts
    // should there be a reward given to the account that opens bid?
    // if so that reward could come from the stake vault, from the open bidder?
    // I am going to explore this thought
    #[account(
        seeds = [b"authority"],
        bump = program_authority.bump
    )]
    pub program_authority: Box<Account<'info, ProgramAuthority>>,

    #[account(
        mut,
        // :: currently can't test since there is no mechanism to change SessionStatus yet.
        // constraint = session.launch_status == SessionStatus::TickBid
        //     @ ErrorCode::InvalidSession, // -=> not correct error code
    )]
    pub session: Box<Account<'info, Session>>,

    #[account(
        mut,
        constraint = tick_bid_round.is_valid_session(session.key().clone())
            @ ErrorCode::InvalidSession,

        constraint = tick_bid_round.is_valid_tick_bid_round(commit_queue.current())
            @ ErrorCode::InvalidTickBidRound,

        constraint = tick_bid_round.is_valid_enqueue_status()
            @ErrorCode::InvalidTickBidRoundStatus,
    )]
    pub tick_bid_round: Box<Account<'info, TickBidRound>>,

    #[account(
        mut,
        constraint = commit_queue.is_valid_session(session.key().clone())
            @ ErrorCode::InvalidSession,

        constraint = commit_queue.is_valid_dequeue()
            @ ErrorCode::IsEmptyQueue,

        constraint = commit_queue.is_valid_open_bid(vested_account_by_owner.owner)
            && commit_queue.is_valid_open_bid(vested_account_by_index.owner)
            @ ErrorCode::InvalidVestedOwner,
    )]
    pub commit_queue: Box<Account<'info, CommitQueue>>,

    #[account(mut)]
    pub vested_config: Box<Account<'info, VestedConfig>>,

    #[account(mut)]
    pub vested_account_by_owner: Box<Account<'info, VestedAccountByOwner>>,

    pub vested_account_by_index: Box<Account<'info, VestedAccountByIndex>>,

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
        mut,
        seeds = [
            session.key().as_ref(),
            b"venture-token-escrow",
        ],
        bump,
    )]
    pub venture_token_escrow: InterfaceAccount<'info, TokenAccount>,

    #[account(
        constraint = program_authority.is_valid_token(bid_token_mint.key().clone())
            @ ErrorCode::InvalidBidToken,
    )]
    pub bid_token_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<OpenBid>) -> Result<()> {
    let OpenBid {
        // signer
        program_authority,

        // get state
        commit_queue,

        // update state
        session,
        tick_bid_round,
        vested_config,
        vested_account_by_owner,

        // token accounts
        commit_bid_vault,
        venture_token_escrow,

        // token mint
        bid_token_mint,

        // program
        token_program,
        ..
    } = ctx.accounts;

    let clock = Clock::get()?;
    let round_index = tick_bid_round.get_index();
    let commit_bid = commit_queue.get();
    let token_amount = 1;

    tick_bid_round.open_bid(clock, commit_bid.amount);

    if !vested_account_by_owner.session_status.is_vested {
        session.add_vested_member();
        vested_config.add_vested_member_by_session();
        vested_account_by_owner.update_vested_by_session();
    }

    if !vested_account_by_owner.round_status[round_index as usize].is_vested {
        vested_config.add_vested_member_by_round(round_index);
        vested_account_by_owner.update_vested_by_round(round_index);
    }

    vested_account_by_owner.update(commit_bid.amount, token_amount, round_index);

    let seeds = &[b"authority", &[program_authority.bump][..]];
    let signer_seeds = &[&seeds[..]];

    transfer_checked(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            TransferChecked {
                from: commit_bid_vault.to_account_info(),
                to: venture_token_escrow.to_account_info(),
                authority: program_authority.to_account_info(),
                mint: bid_token_mint.to_account_info(),
            },
            signer_seeds,
        ),
        commit_bid.amount,
        bid_token_mint.decimals,
    )?;

    commit_bid_vault.reload()?;
    // temp logs
    msg!("AMOUNT: {}", commit_bid.amount);
    msg!("BALANCE: {}", commit_bid_vault.amount);

    commit_queue.dequeue();
    session.update_current_round();
    tick_bid_round.open_round_status();

    Ok(())
}
