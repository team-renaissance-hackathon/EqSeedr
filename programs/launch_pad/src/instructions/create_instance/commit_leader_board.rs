use crate::{
    states::{CommitLeaderBoard, Session},
    utils::*,
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateSessionCommitLeaderBoard<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        // space = CommitLeaderBoard::LEN,
        space = MAX_STATE_ALLOCATION,

        seeds = [
            session.key().as_ref(),
            b"commit-leader-board",
        ],
        bump
    )]
    pub new_commit_leader_board: Account<'info, CommitLeaderBoard>,

    #[account(
        mut,
        has_one = authority,
        // constraint = !session.has_commit_leader_board @ ProgramError::SessionCommitLeaderBoardAlreadyExist
    )]
    pub session: Account<'info, Session>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateSessionCommitLeaderBoard>) -> Result<()> {
    let CreateSessionCommitLeaderBoard {
        new_commit_leader_board,
        session,
        ..
    } = ctx.accounts;

    new_commit_leader_board.initialize(ctx.bumps.new_commit_leader_board, session.key());
    session.add_commit_leader_board();

    Ok(())
}
