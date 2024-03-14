use crate::states::{IndexerStatus, Session};
use crate::utils::{errors::ProgramError, *};
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[derive(Accounts)]
#[instruction(input: SessionParams)]
pub struct CreateSession<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    pub indexer: Account<'info, IndexerStatus>,

    #[account(
        constraint = SessionParams::is_valid_token_name(input.token_name) 
        @ ProgramError::InvalidTokenName,

        constraint = SessionParams::is_valid_token_allocation(input.token_allocation, input.target_rounds)
        @ ProgramError::InvalidTokenAllocation,

        constraint = SessionParams::is_valid_target(input.target_rounds)
        @ ProgramError::InvalidRounds,

        constraint = SessionParams::is_valid_launch_date(input.launch_date)
        @ ProgramError::InvalidLaunchDate,

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

    #[account(
        constraint = token_mint.mint_authority.unwrap() == authority.key()
        @ ProgramError::ExpectMintAuthorityToCreateSession
    )]
    pub token_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct SessionParams {
    token_name: String,
    token_allocation: u64,
    target_rounds: u8,
    launch_date: i64,
}

impl SessionParams {
    const WEEK_DELTA: i64 = 60 * 60 * 24 * 7;

    pub fn is_valid_token_name(token_name: String) -> bool {
        if !(token_name.len() <= 32) {
            return false;
        }

        return true;
    }

    pub fn is_valid_token_allocation(token_allocation: u64, target_rounds: u8) -> bool {
        let target = target_rounds as u64;
        let amount = token_allocation / target;

        if !(amount * target == token_allocation) {
            return false;
        }

        return true;
    }

    pub fn is_valid_target(target: u8) -> bool {
        if !(target >= 4 && target <= 10) {
            return false;
        }

        return true;
    }

    pub fn is_valid_launch_date(launch_date: i64) -> bool {
        // will this be an error if unwrap is not a clock type?
        // need handle this different and return false if is an error
        let clock = Clock::get().unwrap();

        let delta = launch_date - clock.unix_timestamp;

        if !(delta >= SessionParams::WEEK_DELTA) {
            return false;
        }

        return true;
    }
}

pub fn handler(ctx: Context<CreateSession>, input: SessionParams) -> Result<()> {
    let CreateSession {
        indexer,
        new_session,
        token_mint,
        ..
    } = ctx.accounts;

    let clock = Clock::get()?;

    indexer.status.update()?;

    new_session.id = indexer.status.clone();
    new_session.token_mint = token_mint.key();
    new_session.intialized_timestamp = clock.unix_timestamp;

    new_session.token_name = input.token_name;
    new_session.token_allocation = input.token_allocation;
    new_session.target_rounds = input.target_rounds;
    new_session.launch_date = input.launch_date;

    // I think I need to move this out to a different logic process?
    new_session.total_bidders = 0;
    new_session.session_tick = 0;

    emit!(NewSession {
        message: String::from("New Session Created"),
        launch_date: new_session.launch_date,
        session_account: new_session.key(),
        session_indexer: new_session.id.clone(),
    });

    Ok(())
}


#[event]
pub struct NewSession {
    message: String,
    launch_date: i64,
    session_account: Pubkey,
    session_indexer: Indexer,
}
