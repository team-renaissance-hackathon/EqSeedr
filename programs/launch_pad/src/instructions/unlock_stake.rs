use crate::states::{ProgramAuthority, SealedBidByIndex, SealedBidRound, Session};
use crate::utils::errors::ErrorCode;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

#[derive(Accounts)]
pub struct UnlockStake<'info> {
    #[account(mut)]
    pub bidder: Signer<'info>,

    #[account(
        mut,
        constraint = sealed_bid_by_index.owner == bidder.key()
            @ ErrorCode::InvalidOwnerOfSealedBidByIndex,
    )]
    pub sealed_bid_by_index: Account<'info, SealedBidByIndex>,

    #[account(
        mut,
        constraint = bidder_token_account.owner == bidder.key()
    )]
    pub bidder_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        // right now this constraint wont work, no staking account is stored.
        // constraint = session.is_valid_staking_account(session_stake_token_account.key())
    )]
    pub token_stake_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        // right now this constraint wont work since I have to create a cpi so the program authority can be
        // the mint authority.
        // constraint = token_mint.mint_authority.unwrap() == program_authority.key(),
    )]
    pub token_mint: InterfaceAccount<'info, Mint>,

    pub program_authority: Account<'info, ProgramAuthority>,
    pub session: Account<'info, Session>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<UnlockStake>) -> Result<()> {
    let UnlockStake {
        sealed_bid_by_index,
        session,
        bidder_token_account,
        token_stake_vault,
        token_program,
        token_mint,
        program_authority,
        ..
    } = ctx.accounts;

    // Validate that user has unsealed their bid
    require!(sealed_bid_by_index.is_unsealed, ErrorCode::BidNotUnsealed);

    // Validate that the stake isn't already unlocked
    require!(
        !sealed_bid_by_index.is_stake_unlocked,
        ErrorCode::StakeIsAlreadyUnlocked
    );

    // Construct the program authority signer
    let seeds = &[b"auhtority", &[program_authority.bump][..]];
    let signer_seeds = &[&seeds[..]];

    transfer_checked(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            TransferChecked {
                from: token_stake_vault.to_account_info(),
                to: bidder_token_account.to_account_info(),
                authority: program_authority.to_account_info(),
                mint: token_mint.to_account_info(),
            },
            signer_seeds,
        ),
        session.staking_amount,
        token_mint.decimals,
    )?;

    Ok(())
}

// TODO!
// - need to implement event logs
// - add / update validations with correct and working errors
