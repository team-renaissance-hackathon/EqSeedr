use crate::utils::*;
use anchor_lang::prelude::*;

#[account]
pub struct TickBidLeaderBoard {
    pub bump: u8,
    pub session: Pubkey,
    pub pool: LinkedList<Position>,
}

impl TickBidLeaderBoard {
    pub const LEN: usize = BYTE + PUBKEY_BYTES + LinkedList::<Position>::LEN;

    pub fn initialize(&mut self, bump: u8, session: Pubkey) {
        self.bump = bump;
        self.session = session;
        self.pool = LinkedList::<Position>::new();
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct Position {
    pub vested_index: u32,
    pub avg_bid: u64,
}

impl Position {
    pub const LEN: usize = UNSIGNED_32 + UNSIGNED_64;
}

impl Clone for LinkedList<Position> {
    fn clone(&self) -> Self {
        Self {
            total: self.total,
            head: self.head,
            tail: self.tail,
            list: self.list.clone(),
            stack: self.stack.clone(),
        }
    }
}

impl LinkedList<Position> {
    const LEN: usize = 10240 * 10;

    fn new() -> Self {
        Self {
            total: 0,
            head: 0,
            tail: 0,
            list: Vec::<Option<Node<Position>>>::new(),
            stack: Vec::<[u8; 3]>::new(),
        }
    }
}

impl Clone for Node<Position> {
    fn clone(&self) -> Self {
        Self {
            index: self.index,
            prev: self.prev,
            next: self.next,
            position: self.position.clone(),
        }
    }
}

// 4 + (5) + (5) + (4 + 8) = 26 = 100,000 = 2.6 mil
// 3 + (5) + (5) + (3 + 8) = 24 = 100,000 = 2.4 mil
