use anchor_lang::prelude::*;

declare_id!("7GKWqKvkev22SYs2HEb1jw6h4uHJwLVKpEcxVUqTZKxG");

#[program]
pub mod launch_pad {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let Initialize { new_authority, .. } = ctx.accounts;

        new_authority.is_initialzied = true;
        new_authority.is_signer = true;
        new_authority.indexer.init();

        // here to get rid of linter annoyance
        new_authority.indexer.update();

        Ok(())
    }
}

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
            // specify a authority or deployer? hard coded
        ],
        bump
    )]
    pub new_authority: Account<'info, ProgramAuthority>,

    pub system_program: Program<'info, System>,
}

const SIGNED_64: usize = 8;
// const UNSIGNED_128: usize = 16;
const UNSIGNED_64: usize = 8;
// const UNSIGNED_32: usize = 4;
const UNSIGNED_16: usize = 2;
const UNSIGNED_8: usize = 1;
const BOOL: usize = 1;

#[account]
pub struct ProgramAuthority {
    pub is_initialzied: bool,
    pub is_signer: bool,
    pub indexer: Indexer,
}

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
    #[warn(dead_code)]
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

impl ProgramAuthority {
    const LEN: usize = UNSIGNED_64 + BOOL + BOOL + Indexer::LEN;
}
