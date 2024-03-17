#[derive(Accounts)]
pub struct CreateCommitLeaderBoard<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = CommitLeaderBoard::LEN,
        seeds = [
            session.key().as_ref(),
            b"commit-leader-board",
        ],
        bump
    )]
    pub new_commit_leader_board: Account<'info, CommitLeaderBoard>,

    #[account(
        mut,
        has_one = authority
        constraint = !session.has_commit_leader_board @ ProgramError::SessionCommitLeaderBoardAlreadyExist
    )]
    pub session: Account<'info, Session>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateCommitLeaderBoard>) -> Result<()> {
    let CreateCommitLeaderBoard {
        new_commit_leader_board,
        session,
        ..
    } = ctx.accounts;

    // need set state logic

    Ok(())
}
