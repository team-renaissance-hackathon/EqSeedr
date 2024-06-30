use std::borrow::Borrow;

use crate::states::{
    // STATE ACCOUNTS
    Session,
    TickBidRound,
    VestedAccountByIndex,
    VestedAccountByOwner,
    VestedConfig,
};
// use crate::utils::errors::ErrorCode;

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

#[derive(Accounts)]
pub struct ExecuteBid<'info> {
    #[account(mut)]
    pub bid_authority: Signer<'info>,

    // not sure if this is needed or not,
    #[account(
        constraint = vested_account_by_index.owner == vested_account_by_owner.owner,

    )]
    pub vested_account_by_index: Account<'info, VestedAccountByIndex>,

    #[account(
        mut,
        constraint = vested_account_by_owner.session == session.key(),

    )]
    pub vested_account_by_owner: Account<'info, VestedAccountByOwner>,

    #[account(mut)]
    pub session: Account<'info, Session>,

    #[account(
        mut,
        constraint = tick_bid_round.session == session.key(),
        constraint = tick_bid_round.is_valid_open_status(),

        // to help prevent user from uncessary bid, 
        constraint = tick_bid_round.is_valid_delta(),
    )]
    pub tick_bid_round: Account<'info, TickBidRound>,

    #[account(
        mut,
        constraint = vested_config.session == session.key(),
    )]
    pub vested_config: Account<'info, VestedConfig>,

    #[account(mut)]
    pub bid_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [
            session.key().as_ref(),
            b"venture-token-escrow",
        ],
        bump,
    )]
    pub venture_token_escrow: InterfaceAccount<'info, TokenAccount>,

    pub bid_token_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handler(ctx: Context<ExecuteBid>) -> Result<()> {
    let ExecuteBid {
        // signer
        bid_authority,

        // update state
        session,
        tick_bid_round,
        vested_config,
        vested_account_by_owner,

        // token accounts
        bid_ata,
        venture_token_escrow,

        // token mint
        bid_token_mint,

        // program
        token_program,
        ..
    } = ctx.accounts;

    let clock = Clock::get()?;
    let round_index = tick_bid_round.get_index();
    let token_amount = 1;

    let (market_bid, tick_depth) = tick_bid_round.get_current_bid().unwrap();

    if !vested_account_by_owner.session_status.is_vested {
        session.add_vested_member();
        vested_config.add_vested_member_by_session();
        vested_account_by_owner.update_vested_by_session();
    }

    if !vested_account_by_owner.round_status[round_index as usize].is_vested {
        vested_config.add_vested_member_by_round(round_index);
        vested_account_by_owner.update_vested_by_round(round_index);
    }

    vested_account_by_owner.update(market_bid, token_amount, round_index);
    tick_bid_round.update_bid_status(market_bid, tick_depth, clock.borrow());

    transfer_checked(
        CpiContext::new(
            token_program.to_account_info(),
            TransferChecked {
                from: bid_ata.to_account_info(),
                to: venture_token_escrow.to_account_info(),
                authority: bid_authority.to_account_info(),
                mint: bid_token_mint.to_account_info(),
            },
        ),
        market_bid,
        bid_token_mint.decimals,
    )?;

    Ok(())
}
