use crate::states::{CommitQueue, Session};
use crate::utils::errors::ErrorCode;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateSessionCommitQueue<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        constraint = !session.has_commit_queue 
            @ ErrorCode::SessionCommitQueueAlreadyExist,
        init,
        payer = authority,
        space = CommitQueue::LEN,
        seeds = [
            session.key().as_ref(),
            b"commit-queue",
        ],
        bump
    )]
    pub new_commit_queue: Account<'info, CommitQueue>,

    #[account(
        mut,
        has_one = authority
    )]
    pub session: Account<'info, Session>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateSessionCommitQueue>) -> Result<()> {
    let CreateSessionCommitQueue {
        new_commit_queue,
        session,
        ..
    } = ctx.accounts;

    new_commit_queue.initialize(ctx.bumps.new_commit_queue, session.key());
    session.add_commit_queue();

    Ok(())
}
