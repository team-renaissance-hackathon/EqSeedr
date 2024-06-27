use crate::states::{ProgramAuthority, SealedBidByIndex, SealedBidRound, Session};
use crate::utils::errors::ErrorCode;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

#[derive(Accounts)]
pub struct SubmitSealedBid<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [
            b"authority"
        ],
        bump = program_authority.bump,
    )]
    pub program_authority: Account<'info, ProgramAuthority>,

    #[account(
        init,
        payer = authority,
        space = SealedBidByIndex::LEN,
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
        constraint = sealed_bid_round.session == session.key()
            @ ErrorCode::InvalidSealedBidRound,
    )]
    pub sealed_bid_round: Account<'info, SealedBidRound>,

    #[account(
        mut,
        constraint = bidder_token_account.owner == authority.key()
            @ ErrorCode::InvalidTokenOwner,
    )]
    // bidder_token_stake
    pub bidder_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [
            session.key().as_ref(),
            token_stake_mint.key().as_ref(),
            b"token-stake-vault"
        ],
        bump,
    )]
    pub token_stake_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        constraint = token_stake_mint.mint_authority.unwrap() == program_authority.key()
            @ ErrorCode::InvalidMintAuthority,
    )]
    // stake_token_mint
    pub token_stake_mint: InterfaceAccount<'info, Mint>,

    pub session: Account<'info, Session>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SubmitSealedBid>, commit_hash: Pubkey) -> Result<()> {
    let SubmitSealedBid {
        authority,
        new_sealed_bid_by_index,
        sealed_bid_round,
        session,
        bidder_token_account,
        token_stake_vault,
        token_program,
        token_stake_mint,
        ..
    } = ctx.accounts;

    new_sealed_bid_by_index.initialize(
        ctx.bumps.new_sealed_bid_by_index,
        &sealed_bid_round,
        &session,
        authority.key(),
        commit_hash,
    );

    sealed_bid_round.update_total_sealed_bids();

    transfer_checked(
        CpiContext::new(
            token_program.to_account_info(),
            TransferChecked {
                from: bidder_token_account.to_account_info(),
                to: token_stake_vault.to_account_info(),
                authority: authority.to_account_info(),
                mint: token_stake_mint.to_account_info(),
            },
        ),
        session.staking_amount,
        token_stake_mint.decimals,
    )?;

    Ok(())
}

// TODO!
// - Needs update to interface with all SPL token standards and extensions.
// - account inits to reflect anchor 0.30.0 -> may not be relevent since only creating our own accounts
// - need to implement event logs
// - add / update validations with correct and working errors
