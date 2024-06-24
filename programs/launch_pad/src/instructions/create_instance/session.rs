use crate::states::{
    IndexerStatus,
    Session,
    // EnqueueSessionIndex
};
use crate::utils::errors::ErrorCode;
use crate::utils::{MAX_ROUNDS, MAX_TEXT_BYTES, PERCENT_10};
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[derive(Accounts)]
#[instruction(input: SessionParams)]
pub struct CreateSession<'info> {
    #[account(mut
        // need to find a way to compute the fee cost of all state contracts that need to be created
        // so that we can check if the user has enough funds before starting the process
    )]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub indexer: Account<'info, IndexerStatus>,

    #[account(
        constraint = SessionParams::is_valid_token_name(input.token_name)
        @ ErrorCode::InvalidTokenName,

        constraint = !SessionParams::is_valid_token_allocation(input.token_allocation)
        @ ErrorCode::InvalidTokenAllocation,

        constraint = !SessionParams::is_valid_launch_date(input.launch_date)
        @ ErrorCode::InvalidLaunchDate,

        init,
        payer = authority,
        space = Session::LEN,
        seeds = [
            token_mint.key().as_ref(),
            b"session",
        ],
        bump
    )]
    pub new_session: Account<'info, Session>,

    // not going to implement right now
    // pub enqueue_indexer: Account<'info, EnqueueSessionIndex>,
    #[account(
        constraint = token_mint.mint_authority.unwrap() == authority.key()
        @ ErrorCode::ExpectMintAuthorityToCreateSession
    )]
    pub token_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct SessionParams {
    pub token_name: String,
    pub token_allocation: u64,
    pub launch_date: i64,
}

impl SessionParams {
    const WEEK_DELTA: i64 = 60 * 60 * 24 * 7;

    pub fn is_valid_token_name(token_name: String) -> bool {
        return token_name.len() <= MAX_TEXT_BYTES;
    }

    pub fn is_valid_token_allocation(token_allocation: u64) -> bool {
        let amount_10 = token_allocation / PERCENT_10 / MAX_ROUNDS as u64;
        let amount_90 = token_allocation - amount_10;
        let delta = amount_90 / MAX_ROUNDS as u64;

        return !(delta * MAX_ROUNDS as u64 + amount_10 == token_allocation);
    }

    pub fn is_valid_launch_date(launch_date: i64) -> bool {
        let clock = Clock::get().unwrap();
        let delta = launch_date - clock.unix_timestamp;

        return delta < SessionParams::WEEK_DELTA;
    }
}

pub fn handler(ctx: Context<CreateSession>, input: SessionParams) -> Result<()> {
    let CreateSession {
        authority,
        indexer,
        new_session,
        token_mint,
        ..
    } = ctx.accounts;

    let session_pubkey = new_session.key();

    indexer.status.update()?;
    new_session.initialize(
        ctx.bumps.new_session,
        authority.key(),
        indexer.status.clone(),
        token_mint.key(),
        session_pubkey,
        input,
    )?;

    Ok(())
}
