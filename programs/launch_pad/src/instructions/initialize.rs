use super::super::states::{IndexerStatus, ProgramAuthority, SessionIndexer};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    // payer, authority, deployer
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = ProgramAuthority::LEN,
        seeds = [
            b"authority"
        ],
        bump
    )]
    pub new_authority: Account<'info, ProgramAuthority>,

    #[account(
        init,
        payer = authority,
        space = IndexerStatus::LEN,
        seeds = [
            b"indexer-status",
            new_authority.key().as_ref(),
        ],
        bump
    )]
    pub new_indexer_status: Account<'info, IndexerStatus>,

    #[account(
        init,
        payer = authority,
        space = SessionIndexer::LEN,
        seeds = [
            b"active-session-indexer",
            new_authority.key().as_ref(),
        ],
        bump
    )]
    pub new_active_session_indexer: Account<'info, SessionIndexer>,

    #[account(
        init,
        payer = authority,
        space = SessionIndexer::LEN,
        seeds = [
            b"enqueue-session-indexer",
            new_authority.key().as_ref(),
        ],
        bump
    )]
    pub new_enqueue_session_indexer: Account<'info, SessionIndexer>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let Initialize {
        new_authority,
        new_active_session_indexer,
        new_enqueue_session_indexer,
        new_indexer_status,
        ..
    } = ctx.accounts;

    // new_authority.authority = authority.key();
    new_authority.is_initialzied = true;
    new_authority.is_signer = true;

    new_indexer_status.status.init();

    new_authority.bump = ctx.bumps.new_authority;
    new_indexer_status.bump = ctx.bumps.new_indexer_status;
    new_active_session_indexer.bump = ctx.bumps.new_active_session_indexer;
    new_enqueue_session_indexer.bump = ctx.bumps.new_enqueue_session_indexer;

    new_indexer_status.authority = new_authority.key();
    new_active_session_indexer.authority = new_authority.key();
    new_enqueue_session_indexer.authority = new_authority.key();

    Ok(())
}
