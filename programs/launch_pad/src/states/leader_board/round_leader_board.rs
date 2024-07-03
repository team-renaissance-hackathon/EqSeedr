use anchor_lang::prelude::*;

pub struct LeaderBoard {
    pub bump: u8,
    pub session: Pubkey,
    pub round: u8,
    pub data: [u8; 10240 * 10 - (1 + 32 + 1)],
}

impl LeaderBoard {
    pub const LEN: usize = 10240 * 10;
}
