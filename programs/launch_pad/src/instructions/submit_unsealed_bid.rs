#[derive(Accounts)]
#[instructions(index: u32, amount: u64, secret: String)]
pub struct SubmitUnsealedBid<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        constraint = !sealed_bid_by_index.is_valid_unsealed_bid(amount, secret, session.key()),
    )]
    pub sealed_bid_by_index: Account<'info, SealedBidByIndex>,

    #[account(
        mut,
        constraint = sealed_bid_round.authority == session.key(),
        constraint = !sealed_bid_round.is_valid_unsealed_bid_phase(),
        constraint = !sealed_bid_round.is_valid_unsealed_bid(),
    )]
    pub sealed_bid_round: Account<'info, SealedBidRound>,

    #[account(
        mut,
        constraint = commit_leader_board.is_valid_commit_leader_board(sesson.key())
        // linked list validation, is correct index
    )]
    pub commit_leader_board: Account<'info, CommitLeaderBoard>,

    pub session: Account<'info, Session>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SubmitUnsealedBid>, amount: u64) -> Result<()> {
    let SubmitUnsealedBid {
        authority,
        sealed_bid_by_index,
        sealed_bid_round,
        session,
        commit_leader_board,
        ..
    } = ctx.accounts;

    sealed_bid_by_index.unsealed_bid();
    // need index information for linked list
    commit_leader_board.update(sealed_bid_by_index.owner, amount);

    Ok(())
}
