use crate::states::{
    CommitQueue, ProgramAuthority, Session, TickBidRound, VestedAccountByIndex,
    VestedAccountByOwner, VestedConfigBySession,
};
use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct OpenBid<'info> {
    // should the payer be restricted to a specified group?
    // or be open to anyone to execute the open bid?
    // as of right now it is open for everyone.
    #[account(mut)]
    pub payer: Signer<'info>,

    // signer of the commit queue account to transfer funds
    pub program_authority: Account<'info, ProgramAuthority>,

    // I don't think I need this either.
    // I need this
    #[account(mut)]
    pub vested_config: Account<'info, VestedConfigBySession>,

    // I dont' think I need to pass this account in. probably just need the by owner.
    #[account(
        constraint = vested_account_by_index.owner == vested_account_by_owner.owner
    )]
    pub vested_account_by_index: Account<'info, VestedAccountByIndex>,

    #[account(mut)]
    pub vested_account_by_owner: Account<'info, VestedAccountByOwner>,

    // commit bid queue... I think this is a better name that reflects what this is.
    #[account(
        mut,
        constraint = commit_queue.is_valid_dequeue(),
        constraint = commit_queue.is_valid_session(session.key().clone()),
        constraint = commit_queue.is_valid_open_bid(vested_account_by_owner.key().clone()),
    )]
    pub commit_queue: Account<'info, CommitQueue>,

    #[account(
        mut,
        // still there is no mechenism that controls / limits the next tick bid round from starting
        // I need control that in a config contract, should only point to the next round
        // when the current round closes. logically that info should be recorded in the session
        // but i want to give it more thought.
        // or I can increment the pointer only on closing the tick bid round, I think I prefer this method
        constraint = tick_bid_round.is_valid_session(session.key().clone()),
        constraint = tick_bid_round.is_valid_tick_bid_round(commit_queue.current()),
        constraint = tick_bid_round.is_valid_enqueue_status(),
    )]
    pub tick_bid_round: Account<'info, TickBidRound>,

    // I think we don't need this, can directly transfer the funds
    // to the project funding account when submit commit bid.
    // after some more thought, I think this should remain. since
    // duing the commit bid phase, there will be instances where the
    // will be refunded. so that should be seperated from the project
    // fund account.
    // another question is, should there be multiple instances of this
    // account? or a single instance?
    // we need this. for sure... a better name is commit_bid_token_account
    // if it's a single instances, don't have to check session
    // if it's multiple instances, have to check session instance
    // could set this to be multiple instances, that is one to one
    // to the bidder of the given session
    // the authority the program_authority?
    #[account(mut)]
    pub commit_bid_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        // validate the correct project owner
        // constraint = 
    )]
    pub session_bid_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub session: Account<'info, Session>,

    // commit bid token account
    // session funding token account
    // bid token mint
    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}

// may have to wrap state accounts into Box?
pub fn handler(ctx: Context<OpenBid>) -> Result<()> {
    let OpenBid {
        // get state
        commit_queue,

        // update state
        tick_bid_round,
        session,
        vested_config,
        vested_account_by_owner,

        // token accounts
        commit_bid_token_account,
        session_bid_token_account, // project funding account

        // authority
        program_authority,

        // program
        token_program,
        ..
    } = ctx.accounts;

    let clock = Clock::get()?;
    let round_index = tick_bid_round.get_index();
    let commit_bid = commit_queue.get();
    let token_amount = 1;

    tick_bid_round.open_bid(clock, commit_bid.amount);

    // if this is the same as execute bid, then should be execute bid for simplicity
    session.execute_bid(commit_bid.amount, token_amount);

    // should call this vested member instead of vested account? something to think about.
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

    transfer(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            Transfer {
                from: commit_bid_token_account.to_account_info(),
                to: session_bid_token_account.to_account_info(),
                authority: program_authority.to_account_info(),
            },
            &[&[b"authority", &[program_authority.bump]]],
        ),
        commit_bid.amount,
    )?;
    Ok(())
}

// VALIDATION
//  -> is valid session
//  -> session in open status

// UPDATE STATE::
//  tick bid round
//      - status                ->  Open     -> from Enqueue to Open
//      - last market bid       ->  the commit bid queue sets this value
//      - last tick bid depth   ->  value = 0
//      - bid sum               ->  increase value from current commit bid queue
//      - total tokens          ->  increase by one
//      - last bid timestamp    ->  current time of transaction execution
//      - last bid slot         ->  current slot of transaction execution
//      - number of bids        ->  increase by one
//      - init market bid       ->  this is the value that will be used to factor the alogrithm
//                                  to incentize the commit bidder to bid their genuine precieved
//                                  amount that the token should be valued at.
//                                  the idea is 10% above or below is the range, outside thatrange
//                                  creates incentives to other bidders.
//      - computed values:
//          - avg_bid / cost basis
//  session
//      - last market bid   -> need to add
//      - last bidder       -> need to add
//      - number_of_bids    ->  increase by one
//                              other than statistical purpose, what can game theory can be derived from this?
//      - total vested      ->  increase by one if new vested member, not sure if I should track here in the
//                              session contract, would be redundent since it is already being tracked in the
//                              vested config, unless there is a valid reason to do so probably for the leader board?
//      - bid sum           ->  increase value from current commit bid queue
//      - total tokens      ->  increase by one
//      - market value  -> if first round, it sets the value, any other rounds increases this value
//                         this is global value, but not the current value of the tick bid round
//                         so consider different way to handle this in the session contract
//                         or not handle this at all. since it is only relevent, with the
//                         tick bid round. but could consider track the average market value?
//                         whcih is just the computed value -> bid sum / total tokens
//  vested config
//      - session info
//          - vested members    -> only increase if new vested member of that session
//      - round info
//          - vested memebers   -> only increase if new vested member of that round
//  vested account by owner
//      - session info
//          - is_vested     -> first bid sets to true
//          - bid_sum       -> first bid, set value; other bid, increase value
//          - total_tokens  -> increase by one
//      - round info
//          - is_vested     -> first bid sets to true
//          - bid_sum       -> first bid, set value; other bid, increase value
//          - total_tokens  -> increase by one

// INFO
//  - tick bid round
//  - bid amount
//  - token amount
//  - vested account

//  Post instruction
//  update leader baord
//  vested account by index
//      - tick bid leader board index   -> if new, this value will get added
//                                      -> this value will be used for validation
// NOTES:
//      right now this is only taking into consideration the tick bid leader board for
//      the session, but not for the specified round because there is none for the rounds
//      but that looks like that will change since it might be needed since thinking about it more
//      but I am still giving it more thought. but most likely will be a thing.

// if I wanted to build a JS front end framework, how would I go about it?
// what problems would I try to solve?
// considerations
//  - trade offs, something would have to give,
//  - speed
//  - effiency
//  - least amount of work
//  - least amount of memory
//  - least amount of load time
//  - easy dev experience
//  - easy configuration
//  - easy dev tooling
//  - etc
