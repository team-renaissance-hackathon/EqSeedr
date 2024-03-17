#[derive(Accounts)]
pub struct CreateSessionMarketplace<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = MarketMakerPool::LEN,
        seeds = [
            session.key().as_ref(),
            b"marketplace",
        ],
        bump
    )]
    pub new_marketplace: Account<'info, MarketMakerPool>,

    #[account(
        mut,
        has_one = authority
        constraint = !session.has_marketplace @ ProgramError::SessionMarketplaceAlreadyExist
    )]
    pub session: Account<'info, Session>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateSessionMarketplace>) -> Result<()> {
    let CreateSessionMarketplace {
        new_marketplace,
        session,
        ..
    } = ctx.accounts;

    new_marketplace.pool = LinkedList::New();

    session.has_marketplace = true;

    Ok(())
}
