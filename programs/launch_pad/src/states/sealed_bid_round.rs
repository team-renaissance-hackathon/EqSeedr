#[account]
pub struct SealedBidRound {
    pub bump: u8,
    pub authority: Pubkey,
    pub session: Pubkey,

    pub status: Status,

    pub total_sealed_bids: u32,
    pub total_unsealed_bids: u32,
}

impl TickBidRound {
    pub const LEN: usize = DISCRIMINATOR
        + UNSIGNED_8
        + PUBKEY_BYTES
        + PUBKEY_BYTES
        + Status::LEN
        + UNSIGNED_32
        + UNSIGNED_32;

    pub fn initialize(&mut self, bump: u8, authority: Pubkey, session: Session) {
        self.bump = bump;
        self.authority = authority;
        self.session = session;

        self.status = Status::Enqueue;

        self.total_sealed_bids = 0;
        self.total_unsealed_bids = 0;

        // emit event
    }
}

pub enum Status {
    Enqueue,
    Open,
    Closed,
    Canceled,
}

impl Status {
    const LEN: usize = BYTE;
}

// sealed bid system
//  SealedBidRound
//  CommitQueue
//  CommitLeaderBoard
