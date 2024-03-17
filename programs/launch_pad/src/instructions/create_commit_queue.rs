#[derive(Accounts)]
pub struct CreateCommitQueue<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = CommitQueue::LEN,
        seeds = [
            session.key().as_ref(),
            b"commit-queue",
        ],
        bump
    )]
    pub new_commit_leader_board: Account<'info, CommitQueue>,

    #[account(
        mut,
        has_one = authority
        constraint = !session.has_commit_queue @ ProgramError::SessionCommitQueueAlreadyExist
    )]
    pub session: Account<'info, Session>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateCommitQueue>) -> Result<()> {
    let CreateCommitQueue {
        new_commit_leader_board,
        session,
        ..
    } = ctx.accounts;

    // need set state logic

    Ok(())
}
