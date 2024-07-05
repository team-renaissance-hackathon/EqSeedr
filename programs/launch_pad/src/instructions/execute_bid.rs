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

        // total tokens == token allocation
        constraint = !tick_bid_round.is_complete(),
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
    // session.update_bid_status(market);

    // execute tick depth algo
    let pool_tokens = tick_bid_round.update_pool_simple(tick_depth);
    let (total_tokens, pool_tokens) =
        if (pool_tokens + token_amount) > tick_bid_round.token_allocation {
            let amount = tick_bid_round.token_allocation
                - (tick_bid_round.total_tokens + tick_bid_round.bonus_pool);
            (amount, amount - token_amount)
        } else {
            (pool_tokens + token_amount, pool_tokens)
        };
    // session
    //  increse bid sum
    session.bid_sum += market_bid;
    //  increase total tokens
    session.total_tokens += total_tokens;
    //  compute average bid
    //      avg = session.bid_sum / session.total_tokens
    //  compute remainining tokens
    //      reamining = session.token_allocation - session.total_tokens
    // number of vested accounts -> total_vested
    // number of bids -> number_of_bids
    // market value -> market_value -> would it be the same as computed average bid?

    // round
    //  increase bid sum
    tick_bid_round.bid_sum += market_bid;
    //  increase total tokens
    tick_bid_round.total_tokens += token_amount;
    //  update pool
    tick_bid_round.bonus_pool += pool_tokens;
    //  compute average bid -> going with option b
    //      option a :: avg = tick_bid_round.bid_sum / (tick_bid_round.total_tokens + tick_bid_round.bonus_pool)
    //      option b :: avg = tick_bid_round.bid_sum / tick_bid_round.total_tokens
    //  compute remainining tokens
    //      remaining = tick_bid_round.token_allocation - (tick_bid_round.total_tokens + tick_bid_round.bonus_pool)
    //  update highest bid
    tick_bid_round.update_highest_bid_rank(market_bid, vested_account_by_owner.bid_index);
    //  update nearest avg bid
    //  update biggest tick depth
    // if tick_depth >= tick_bid_round.curent_tick_depth {
    //     tick_bid_round.curent_tick_depth = tick_depth;
    //     tick_bid_round.tick_depth_index = vested_account_by_owner.bid_index;
    // }
    //  update biggest tick depth acumulation
    //  increase tick depth accumulation
    tick_bid_round.tick_depth_accumulation += tick_depth;

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

// update highest overall bid
// update avg bid leader board
// need to keep track of tick depth and sum of tick depth
// need to keep track of the investor avg bid closest to round avg bid
// close round after last bid
// update rankings for bonus pool
