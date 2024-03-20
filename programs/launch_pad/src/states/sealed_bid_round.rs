use crate::utils::*;
use anchor_lang::prelude::*;

#[account]
pub struct SealedBidRound {
    pub bump: u8,
    pub authority: Pubkey,
    pub session: Pubkey,

    status: Status,

    pub total_sealed_bids: u32,
    pub total_unsealed_bids: u32,
}

impl SealedBidRound {
    pub const LEN: usize = DISCRIMINATOR
        + UNSIGNED_8
        + PUBKEY_BYTES
        + PUBKEY_BYTES
        + Status::LEN
        + UNSIGNED_32
        + UNSIGNED_32;

    pub fn initialize(&mut self, bump: u8, authority: Pubkey, session: Pubkey) {
        self.bump = bump;
        self.authority = authority;
        self.session = session;

        self.status = Status::Enqueue;

        self.total_sealed_bids = 0;
        self.total_unsealed_bids = 0;

        // emit event
    }

    pub fn next_index(&self) -> String {
        return (self.total_sealed_bids + 1).to_string();
    }

    pub fn get_index(&self) -> u32 {
        return self.total_sealed_bids + 1;
    }

    pub fn update_total_sealed_bids(&mut self) {
        self.total_sealed_bids += 1;
    }

    pub fn update_total_unsealed_bids(&mut self) {
        self.total_unsealed_bids += 1;
    }

    // VALIDATIONS:
    pub fn is_valid_stake_amount(&self) -> bool {
        return true;
    }

    pub fn is_valid_sealed_bid_phase(&self) -> bool {
        match self.status {
            Status::SealedBidPhase => !true,
            _ => !false,
        }
    }

    pub fn is_valid_unsealed_bid_phase(&self) -> bool {
        match self.status {
            Status::UnsealBidPhase => !true,
            _ => !false,
        }
    }

    // don't think this is necessary, will combine the commit bid phase with unsealed bid phase
    pub fn is_valid_commit_bid_phase(&self) -> bool {
        return true;
    }

    pub fn is_valid_commit_amount(&self) -> bool {
        // check user balance
        return true;
    }

    pub fn has_not_commit(&self) -> bool {
        return true;
    }

    pub fn is_valid_sealed_bid_round(&self) -> bool {
        // use unix timestamp || status
        return true;
    }

    pub fn is_valid_unsealed_bid(&self) -> bool {
        return self.total_unsealed_bids < self.total_sealed_bids;
    }

    // pub fn is_valid() {}
}

#[account]
pub struct CommitLeaderBoard {
    pub bump: u8,
    pub session: Pubkey,
    pub min_target: u64, // cutoff / bottom amount, increaese when commit queue has 10 -> I don't think I need this
    pub pool: CommitLeaderBoardLinkedList,
}

impl CommitLeaderBoard {
    pub const LEN: usize =
        DISCRIMINATOR + BUMP + PUBKEY_BYTES + UNSIGNED_64 + CommitLeaderBoardLinkedList::LEN;
}

impl CommitLeaderBoard {
    pub fn initialize(&mut self, bump: u8, session: Pubkey) {
        self.bump = bump;
        self.session = session;
        self.min_target = 0;

        self.pool = CommitLeaderBoardLinkedList::new();
    }

    // pub fn update(&mut self, owner: Pubkey, amount: u64) {
    //     // add this code later. going to need index info for linked list
    //     // self.pool;
    // }

    pub fn is_valid_commit_leader_board(&mut self, session: Pubkey) -> bool {
        return self.session == session;
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct CommitLeaderBoardLinkedList {
    pub total: u32,
    head: u32,
    tail: u32,
    list: Vec<CommitNode>,
    stack: Vec<[u8; 3]>,
}

impl CommitLeaderBoardLinkedList {
    pub const LEN: usize = UNSIGNED_32
        + UNSIGNED_32
        + UNSIGNED_32
        + (BYTE + (BYTE + CommitNode::LEN) * MAX_PARTICPANTS)
        + (BYTE + 3 * MAX_PARTICPANTS);

    pub fn new() -> Self {
        CommitLeaderBoardLinkedList {
            total: 0,
            head: 0,
            tail: 0,
            list: Vec::<CommitNode>::new(),
            stack: Vec::<[u8; 3]>::new(),
        }
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct CommitNode {
    index: u32,
    prev: Option<u32>,
    next: Option<u32>,
    position: Commit,
}

impl CommitNode {
    pub const LEN: usize = UNSIGNED_32 + (BYTE + UNSIGNED_32) + (BYTE + UNSIGNED_32) + Commit::LEN;
}

#[account]
pub struct CommitQueue {
    pub bump: u8,
    pub session: Pubkey,
    pointer: u8,
    queue: Vec<Commit>,
}

const MAX_CAPACITY: usize = 10;
impl CommitQueue {
    pub const LEN: usize =
        DISCRIMINATOR + BUMP + PUBKEY_BYTES + BYTE + (UNSIGNED_128 + (Commit::LEN * MAX_CAPACITY));

    pub fn initialize(&mut self, bump: u8, session: Pubkey) {
        self.bump = bump;
        self.session = session;
        self.queue = Vec::new();

        // emit event
    }

    pub fn insert(&mut self, commit: Commit) {
        let mut index = self.queue.len();

        while index != 0 && commit.amount > self.queue[index - 1].amount {
            index -= 1;
        }

        if self.queue.len() != 0 && self.queue.len() == MAX_CAPACITY {
            self.queue.insert(index, commit);
            self.queue.pop();
        } else if self.queue.len() != 0 && index < MAX_CAPACITY && index < self.queue.len() {
            self.queue.insert(index, commit);
        } else {
            self.queue.push(commit);
        }

        // emit event element was added
    }

    pub fn dequeue(&mut self) -> Commit {
        let index = self.pointer;
        self.pointer += 1;
        return self.queue[index as usize].clone();
    }

    pub fn is_valid_insert(&self, commit: Commit) -> bool {
        return self.queue.len() == MAX_CAPACITY
            && commit.amount > self.queue[self.queue.len() - 1].amount;
    }

    pub fn is_valid_dequeue(&self) -> bool {
        return self.pointer < MAX_CAPACITY as u8;
    }

    pub fn is_valid_session(&self, session: Pubkey) -> bool {
        return self.session == session;
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct Commit {
    pub bidder_index: u32,
    pub amount: u64,
}

impl Commit {
    const LEN: usize = UNSIGNED_32 + UNSIGNED_64;
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
enum Status {
    Enqueue,
    SealedBidPhase,
    UnsealBidPhase,
    Canceled,
}

impl Status {
    const LEN: usize = BYTE;
}

// sealed bid system
//  SealedBidRound
//  SealedBidByIndex
//  CommitLeaderBoard
//  CommitQueue

// temperary
pub trait Len {
    const LEN: usize;
}
