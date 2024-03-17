#[derive(Accounts)]
pub struct CreateSealedBidRound<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = SealedBidRound::LEN,
        seeds = [
            session.key().as_ref(),
            b"sealed-bid-round",
        ],
        bump
    )]
    pub new_sealed_bid: Account<'info, SealedBidRound>,

    #[account(
        mut,
        has_one = authority
        constraint = !session.has_marketplace @ ProgramError::SessionSealedBidRoundAlreadyExist
    )]
    pub session: Account<'info, Session>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateSealedBidRound>) -> Result<()> {
    let CreateSealedBidRound {
        new_sealed_bid_round,
        session,
        ..
    } = ctx.accounts;

    // need set state logic

    Ok(())
}
