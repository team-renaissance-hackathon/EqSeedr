use anchor_lang::{prelude::*, solana_program::pubkey::PUBKEY_BYTES};

declare_id!("7GKWqKvkev22SYs2HEb1jw6h4uHJwLVKpEcxVUqTZKxG");

#[program]
pub mod launch_pad {
    use super::*;

    // LOGIC

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
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

        new_indexer_status.indexer.init();

        // here to get rid of linter annoyance
        new_indexer_status.indexer.update();

        new_authority.bump = ctx.bumps.new_authority;
        new_indexer_status.bump = ctx.bumps.new_indexer_status;
        new_active_session_indexer.bump = ctx.bumps.new_active_session_indexer;
        new_enqueue_session_indexer.bump = ctx.bumps.new_enqueue_session_indexer;

        new_indexer_status.authority = new_authority.key();
        new_active_session_indexer.authority = new_authority.key();
        new_enqueue_session_indexer.authority = new_authority.key();

        Ok(())
    }
}

// UTILS

// VALIDATIONS

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
        space = ProgramAuthority::LEN,
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

// CONSTANTS

const DISCRIMINATOR: usize = 8;
const SIGNED_64: usize = 8;
// const UNSIGNED_128: usize = 16;
// const UNSIGNED_64: usize = 8;
// const UNSIGNED_32: usize = 4;
const UNSIGNED_16: usize = 2;
const UNSIGNED_8: usize = 1;
const BOOL: usize = 1;
const BUMP: usize = 1;

// ACCOUNTS

#[account]
pub struct ProgramAuthority {
    pub bump: u8,

    // specify a authority or deployer?
    // pub authority: Pubkey,
    pub is_initialzied: bool,
    pub is_signer: bool,
}

#[account]
pub struct IndexerStatus {
    pub bump: u8,
    pub authority: Pubkey,
    pub indexer: Indexer,
}

#[account]
pub struct SessionIndexer {
    pub bump: u8,
    pub authority: Pubkey,

    // not sure this will work in anchor
    pub list: Vec<(Indexer, Pubkey)>,
}

impl ProgramAuthority {
    const LEN: usize = DISCRIMINATOR + BUMP + BOOL + BOOL;
}

impl IndexerStatus {
    const LEN: usize = DISCRIMINATOR + BUMP + PUBKEY_BYTES + Indexer::LEN;
}

impl SessionIndexer {
    const LIMIT: usize = 100;
    const LEN: usize =
        DISCRIMINATOR + BUMP + PUBKEY_BYTES + (Indexer::LEN + PUBKEY_BYTES) * SessionIndexer::LIMIT;
}

// TYPES

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct Indexer {
    year_timestamp: i64,
    week_timestamp: i64,
    day_timestamp: i64,
    year: u16,
    week: u8,
    nonce: u8,
    delta_index: u8,
}

impl Indexer {
    const LEN: usize =
        SIGNED_64 + SIGNED_64 + SIGNED_64 + UNSIGNED_16 + UNSIGNED_8 + UNSIGNED_8 + UNSIGNED_8;

    // will work up to year 2038, this needs an update when a new standard exist.
    const YEAR_DELTA: [i64; 4] = [31_622_400, 31_536_000, 31_536_000, 31_536_000];
    const WEEK_DELTA: i64 = 604_800;
    const DAY_DELTA: i64 = 86_400;

    const GENISUS_TIMESTAMP: i64 = 1_704_067_200;
    const INIT_YEAR: u16 = 2024;

    fn init(&mut self) {
        self.year_timestamp = Indexer::GENISUS_TIMESTAMP;
        self.week_timestamp = Indexer::GENISUS_TIMESTAMP;
        self.day_timestamp = Indexer::GENISUS_TIMESTAMP;
        self.year = Indexer::INIT_YEAR;
        self.week = 0;
        self.nonce = 0;
        self.delta_index = 0;
    }

    fn update(&mut self) {
        let clock = Clock::get().unwrap();
        let index = self.delta_index as usize;

        if clock.unix_timestamp - self.year_timestamp >= Indexer::YEAR_DELTA[index] {
            self.year_timestamp += Indexer::YEAR_DELTA[index];
            self.year += 1;
            self.week = 0;
            self.nonce = 0;
            self.delta_index = (self.delta_index + 1) % Indexer::YEAR_DELTA.len() as u8;
            return;
        }

        if clock.unix_timestamp - self.week_timestamp >= Indexer::WEEK_DELTA {
            self.week_timestamp += Indexer::WEEK_DELTA;
            self.week += 1;
            self.nonce = 0;
            return;
        }

        if clock.unix_timestamp - self.day_timestamp >= Indexer::DAY_DELTA {
            self.day_timestamp += Indexer::DAY_DELTA;
            self.nonce = 0;
            return;
        }

        self.nonce += 1;
    }
}
