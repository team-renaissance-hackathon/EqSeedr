use crate::{
    states::{Session, MarketplacePositions},
    // utils::ProgramError,
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateSessionMarketplacePositions<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = MarketplacePositions::LEN,
        seeds = [
            session.key().as_ref(),
            b"session-marketplace",
        ],
        bump
    )]
    pub new_marketplace_positions: Account<'info, MarketplacePositions>,

    #[account(
        mut,
        has_one = authority,
        constraint = !session.has_marketplace_positions 
        // @ ProgramError::SessionMarketplacePositionsAlreadyExist
    )]
    pub session: Account<'info, Session>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateSessionMarketplacePositions>) -> Result<()> {
    let CreateSessionMarketplacePositions {
        new_marketplace_positions,
        session,
        ..
    } = ctx.accounts;

    new_marketplace_positions.initialize();
    session.add_marketplace_positions();

    Ok(())
}
