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
    pub new_sealed_bid_round: Account<'info, SealedBidRound>,

    #[account(
        mut,
        has_one = authority
        constraint = !session.has_sealed_bid_round @ ProgramError::SessionSealedBidRoundAlreadyExist
    )]
    pub session: Account<'info, Session>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateSealedBidRound>) -> Result<()> {
    let CreateSealedBidRound {
        authority,
        new_sealed_bid_round,
        session,
        ..
    } = ctx.accounts;

    new_sealed_bid_round.initialize(
        ctx.bumps.new_sealed_bid_round,
        authority.key().clone(),
        session.key().clone(),
    );

    Ok(())
}
