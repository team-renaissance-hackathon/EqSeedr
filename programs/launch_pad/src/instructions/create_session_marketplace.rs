use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateSessionMarketplace<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    // do I need program authority? I don't think so unless it should be tied to seeds
    // #[account(
    //     seeds = [
    //         b"authority",
    //     ],
    //     bump = program_authority.bump
    // )]
    // pub program_authority: Account<'info, ProgramAuthority>,
    #[account(
        init,
        payer = authority,
        space = MarketMakerPool::LEN,
        seeds = [
            session.key().as_ref(),
            // program authority?
            b"marketplace",
        ],
        bump
    )]
    pub new_marketplace: Account<'info, MarketMakerPool>,

    #[account(
        mut,
        constraint = !session.has_marketplace // @ ProgramError::SessionMarketplaceAlreadyExist
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

    new_marketplace.next_bid = 0;
    new_marketplace.queue = Vec::New();

    session.has_marketplace = true;

    Ok(())
}
