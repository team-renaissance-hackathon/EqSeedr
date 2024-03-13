use super::constants::*;
use anchor_lang::prelude::*;

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
    pub const LEN: usize =
        SIGNED_64 + SIGNED_64 + SIGNED_64 + UNSIGNED_16 + UNSIGNED_8 + UNSIGNED_8 + UNSIGNED_8;

    // will work up to year 2038, this needs an update when a new standard exist.
    const YEAR_DELTA: [i64; 4] = [31_622_400, 31_536_000, 31_536_000, 31_536_000];
    const WEEK_DELTA: i64 = 604_800;
    const DAY_DELTA: i64 = 86_400;

    const GENISUS_TIMESTAMP: i64 = 1_704_067_200;
    const INIT_YEAR: u16 = 2024;

    pub fn init(&mut self) {
        self.year_timestamp = Indexer::GENISUS_TIMESTAMP;
        self.week_timestamp = Indexer::GENISUS_TIMESTAMP;
        self.day_timestamp = Indexer::GENISUS_TIMESTAMP;
        self.year = Indexer::INIT_YEAR;
        self.week = 0;
        self.nonce = 0;
        self.delta_index = 0;
    }

    pub fn update(&mut self) {
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
