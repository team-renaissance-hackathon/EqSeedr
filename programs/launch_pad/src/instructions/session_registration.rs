#[derive(Accounts)]
pub struct SessionRegistration<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = VestedAccountByOwner::LEN,
        seeds = [
            auhtority.key().as_ref(),
            session.key().as_ref(),
            b"vested-account-by-owner",
        ],
        bump
    )]
    pub new_vested_account_by_owner: Account<'info, VestedAccountByOwner>,

    #[account(
        init,
        payer = authority,
        space = VestedAccountByIndex::LEN,
        seeds = [
            auhtority.key().as_ref(),
            session.key().as_ref(),
            b"vested-account-by-index",
        ],
        bump
    )]
    pub new_vested_account_by_index: Account<'info, VestedAccountByIndex>,

    #[account{
        mut,
        constraint = tick_bid_leader_board.session_id == session.key()
    }]
    pub tick_bid_leader_board: Account<'info, TickBidLeaderBoard>,

    pub session: Account<'info, Session>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SessionRegistration>) -> Result<()> {
    let SessionRegistration {
        authority,
        new_vested_account_by_owner,
        new_vested_account_by_index,
        tick_bid_leader_board,
        session,
        ..
    } = ctx.accounts;

    // need set state logic

    Ok(())
}
