use anchor_lang::prelude::*;

use crate::states::{
    Session,
    // TickBidLeaderBoard,
    VestedAccountByIndex,
    VestedAccountByOwner,
    VestedConfigBySession,
};

use anchor_spl::token::Mint;

#[derive(Accounts)]
pub struct SessionRegistration<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = VestedAccountByOwner::LEN,
        seeds = [
            authority.key().as_ref(),
            session.clone().key().as_ref(),
            b"vested-account-by-owner",
        ],
        bump
    )]
    pub new_vested_account_by_owner: Box<Account<'info, VestedAccountByOwner>>,

    #[account(
        init,
        payer = authority,
        space = VestedAccountByIndex::LEN,
        seeds = [
            vested_config.next_index().as_ref(),
            session.clone().key().as_ref(),
            b"vested-account-by-index",
        ],
        bump
    )]
    pub new_vested_account_by_index: Box<Account<'info, VestedAccountByIndex>>,

    // #[account{
    //     mut,
    //     constraint = tick_bid_leader_board.session == session.key()
    // }]
    // pub tick_bid_leader_board: Account<'info, TickBidLeaderBoard>,
    #[account(
        mut,
        constraint = vested_config.session == session.key()
    )]
    pub vested_config: Account<'info, VestedConfigBySession>,

    #[account(
        mut,
        constraint = session.token_mint == token_mint.key()
    )]
    pub session: Account<'info, Session>,

    pub token_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SessionRegistration>) -> Result<()> {
    let SessionRegistration {
        authority,
        vested_config,
        new_vested_account_by_owner,
        new_vested_account_by_index,
        // tick_bid_leader_board,
        session,
        ..
    } = ctx.accounts;

    // vested_config.update_index();

    new_vested_account_by_index.initialize(
        ctx.bumps.new_vested_account_by_index,
        vested_config.vested_index,
        authority.key(),
        session.key(),
        vested_config.key(),
        // tick_bid_leader_board,
    );

    new_vested_account_by_owner.initialize(
        ctx.bumps.new_vested_account_by_owner,
        vested_config.vested_index,
        authority.key(),
        session.key(),
        vested_config.key(),
        // tick_bid_leader_board,
    );

    // I have thoughts on this...
    // need to revisit this
    // buecause I have questions...
    // right now this is the session tick bid leader board
    // the more I come across this, it is becoming more
    // apparent that I also need a leader baord for each round too
    // also the leader board will heavily tied to the algorithm
    // so the structure of the leader board may need to be reconsidered
    // to reflect the algorithm of the bonus bag
    // tick_bid_leader_board.add();

    Ok(())
}

// should there be a leader board for every round?
