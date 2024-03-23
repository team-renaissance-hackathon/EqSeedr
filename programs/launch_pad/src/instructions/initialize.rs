use super::super::states::{
    ActiveSessionIndex, EnqueueSessionIndex, IndexerStatus, MarketplaceMatchers, ProgramAuthority,
    SessionIndexer,
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

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
        seeds = [
            new_authority.key().as_ref(),
            b"token-mint",
        ],
        bump,
        mint::authority = new_authority,
        mint::decimals = 9,
        mint::freeze_authority = new_authority,
    )]
    pub new_token_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        associated_token::mint = new_token_mint,
        associated_token::authority = new_authority,
        associated_token::token_program = token_program,
    )]
    pub new_authority_token_account: Account<'info, TokenAccount>,

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
    pub new_active_session_indexer: Account<'info, ActiveSessionIndex>,

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
    pub new_enqueue_session_indexer: Account<'info, EnqueueSessionIndex>,

    #[account(
        init,
        payer = authority,
        space = MarketplaceMatchers::LEN,
        seeds = [
            b"marketplace-matchers",
            new_authority.key().as_ref(),
        ],
        bump
    )]
    pub new_marketplace_matchers: Account<'info, MarketplaceMatchers>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let Initialize {
        new_authority,
        new_indexer_status,
        new_active_session_indexer,
        new_enqueue_session_indexer,
        new_marketplace_matchers,
        ..
    } = ctx.accounts;

    msg!(
        " BUMPS: {}, {}, {}, {}, {}",
        ctx.bumps.new_authority,
        ctx.bumps.new_indexer_status,
        ctx.bumps.new_active_session_indexer,
        ctx.bumps.new_enqueue_session_indexer,
        ctx.bumps.new_marketplace_matchers
    );

    // new_authority.authority = authority.key();???
    // for some reason, bumps is broken when I initialize all these state contracts
    // so I am hard coding the bumps in, at least for now until
    // I figure out how to resolve this bumps issue.
    // I speculate that it's a bug with anchor of some kind.
    new_authority.initialize(255);

    new_indexer_status.initialize(253, new_authority.key());

    new_active_session_indexer.initialize(255, new_authority.key());

    new_enqueue_session_indexer.initialize(255, new_authority.key());

    new_marketplace_matchers.initialize(254, new_authority.key());

    // emit event ->
    //  -> MSG!program deployed and initialized,
    //  -> list all accounts

    Ok(())
}
