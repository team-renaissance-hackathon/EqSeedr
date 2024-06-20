use crate::utils::*;
use anchor_lang::prelude::*;

use super::SealedBidByIndex;

#[account]
pub struct SealedBidRound {
    pub bump: u8,
    pub authority: Pubkey,
    pub session: Pubkey,

    pub status: SealedBidRoundStatus,

    pub total_sealed_bids: u32,
    pub total_unsealed_bids: u32,
}

impl SealedBidRound {
    pub const LEN: usize = DISCRIMINATOR
        + BYTE
        + PUBKEY_BYTES
        + PUBKEY_BYTES
        + SealedBidRoundStatus::LEN
        + UNSIGNED_32
        + UNSIGNED_32;

    pub fn initialize(&mut self, bump: u8, authority: Pubkey, session: Pubkey) {
        self.bump = bump;
        self.authority = authority;
        self.session = session;

        self.status = SealedBidRoundStatus::Enqueue;

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

    // currently not testing since would need a
    // transaction to set this value.
    pub fn is_valid_sealed_bid_phase(&self) -> bool {
        match self.status {
            SealedBidRoundStatus::SealedBidPhase => !true,
            _ => !false,
        }
    }

    // currently not testing since would need a
    // transaction to set this value.
    // should consider the constraint by messure of time
    pub fn is_valid_unsealed_bid_phase(&self) -> bool {
        match self.status {
            SealedBidRoundStatus::UnsealBidPhase => !true,
            _ => !false,
        }
    }

    // -- SOME OF THESE VALIDATIONS SEAM INCOMPLETE ATM. need to explore implementing them.
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
        return !(self.total_unsealed_bids < self.total_sealed_bids);
    }

    pub fn is_valid_session(&self, session: Pubkey) -> bool {
        return !(self.session == session);
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

    pub fn get_node(&self, index: u32) -> Commit {
        // index -> commit_leader_board_index
        // should validate that we grab the correct index of commit
        // position.commit.bid_index == sealed_bid_index.bid_index
        return self.pool.list[index as usize].clone().unwrap().position;
    }

    pub fn create_node(&self, bid_index: u32, amount: u64) -> CommitNode {
        CommitNode {
            index: self.pool.total,
            prev: None,
            next: None,
            position: Commit { bid_index, amount },
        }
    }

    pub fn add(&mut self, node: &mut CommitNode, index: u32) {
        self.pool.add(index, node);
    }

    pub fn is_valid_session(&self, session: Pubkey) -> bool {
        return !(self.session == session);
    }

    pub fn is_valid_node(&self, pos: u32, amount: u64) -> bool {
        return self.pool.node_is_valid(pos, amount);
    }

    pub fn is_valid_indexed_commit_bid(
        &self,
        sealed_bid_by_index: &Account<SealedBidByIndex>,
    ) -> bool {
        return !(self.pool.list[sealed_bid_by_index.commit_leader_board_index as usize]
            .clone()
            .unwrap()
            .index
            == sealed_bid_by_index.bid_index);
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct CommitLeaderBoardLinkedList {
    pub total: u32,
    head: u32,
    tail: u32,
    list: Vec<Option<CommitNode>>,
    stack: Vec<[u8; 3]>,
}

impl CommitLeaderBoardLinkedList {
    const STACK: usize = 3;
    pub const LEN: usize = UNSIGNED_32
        + UNSIGNED_32
        + UNSIGNED_32
        + (BYTE + (BYTE + CommitNode::LEN) * MAX_PARTICPANTS)
        + (BYTE + CommitLeaderBoardLinkedList::STACK * MAX_PARTICPANTS);

    pub fn new() -> Self {
        CommitLeaderBoardLinkedList {
            total: 0,
            head: 0,
            tail: 0,
            list: Vec::<Option<CommitNode>>::new(),
            stack: Vec::<[u8; 3]>::new(),
        }
    }

    // fn next(&self, node: CommitNode) -> Option<CommitNode> {
    //     match node.next {
    //         Some(pos) => self.list[pos as usize].clone(),
    //         None => None,
    //     }
    // }

    // fn prev(&self, node: CommitNode) -> Option<CommitNode> {
    //     match node.prev {
    //         Some(pos) => self.list[pos as usize].clone(),
    //         None => None,
    //     }
    // }

    fn insert(&mut self, pos: u32, node: &mut CommitNode) {
        if self.total == 0 {
            node.index = 0;
            self.head = node.index;
        } else if pos == self.head {
            let next_node = &mut self.list[self.head as usize].clone().unwrap();

            self.head = node.index;
            node.prev = None;
            node.next = Some(next_node.index);
            next_node.prev = Some(node.index);

            self.list[next_node.index as usize] = Some(next_node.clone());
        } else if pos >= self.total || self.list[pos as usize].is_none() {
            let prev_node = &mut self.list[self.tail as usize].clone().unwrap();

            self.tail = node.index;
            node.prev = Some(prev_node.index);
            node.next = None;
            prev_node.next = Some(node.index);

            self.list[prev_node.index as usize] = Some(prev_node.clone());
        } else {
            let next_node = &mut self.list[pos as usize].clone().unwrap();
            let prev_node = &mut self.list[next_node.prev.clone().unwrap() as usize]
                .clone()
                .unwrap();

            prev_node.next = Some(node.index);
            next_node.prev = Some(node.index);
            node.prev = Some(prev_node.index);
            node.next = Some(next_node.index);

            next_node.prev = Some(node.index);
            prev_node.next = Some(node.index);

            self.list[next_node.index as usize] = Some(next_node.clone());
            self.list[prev_node.index as usize] = Some(prev_node.clone());
        }
    }

    pub fn update(&mut self, node: CommitNode) {
        self.list[node.index as usize] = Some(node.clone());
    }

    pub fn remove(&self, pos: u32) -> CommitNode {
        return self.list[pos as usize].clone().unwrap();
    }

    pub fn set_to_none(&mut self, pos: u32) {
        let node = self.remove(pos);
        self.push(node.index);
        self.list[node.index as usize] = None;
    }

    pub fn add(&mut self, pos: u32, node: &mut CommitNode) {
        if !self.stack.is_empty() {
            node.index = self.pop();
            self.list[node.index as usize] = Some(node.clone());
            self.insert(pos, node);
        } else {
            node.index = self.total;
            self.insert(pos, node);
            self.list.push(Some(node.clone()));
            self.total += 1;
        }
    }

    fn push(&mut self, pos: u32) {
        let u32_as_bytes: [u8; 4] = pos.to_be_bytes();
        let u24_as_bytes: [u8; 3] = [u32_as_bytes[1], u32_as_bytes[2], u32_as_bytes[3]];
        self.stack.push(u24_as_bytes);
    }

    fn pop(&mut self) -> u32 {
        let u24_as_bytes: [u8; 3] = self.stack.pop().unwrap();
        let u32_as_bytes: [u8; 4] = [0, u24_as_bytes[0], u24_as_bytes[1], u24_as_bytes[2]];
        return u32::from_be_bytes(u32_as_bytes);
    }

    pub fn last_element(&self) -> u32 {
        let u24_as_bytes: [u8; 3] = self.stack[self.stack.len() - 1];
        let u32_as_bytes: [u8; 4] = [0, u24_as_bytes[0], u24_as_bytes[1], u24_as_bytes[2]];
        return u32::from_be_bytes(u32_as_bytes);
    }

    fn node_is_valid(&self, pos: u32, amount: u64) -> bool {
        if self.total == 0 {
            return !true;
        } else if pos == self.head {
            let head_node = self.list[self.head as usize].as_ref().unwrap();

            return !(amount > head_node.position.amount);
        } else if (!self.stack.is_empty() && self.list[pos as usize].is_none()) || pos >= self.total
        {
            let tail_node = self.list[self.tail as usize].as_ref().unwrap();

            return !(amount <= tail_node.position.amount);
        } else {
            let next_node = self.list[pos as usize].as_ref().unwrap();
            let prev_node = self.list[next_node.prev.unwrap() as usize]
                .as_ref()
                .unwrap();

            return !(amount > next_node.position.amount && amount <= prev_node.position.amount);
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

// should change to name it CommitBidQueue
#[account]
pub struct CommitQueue {
    pub bump: u8,
    pub session: Pubkey,

    // seems like the private field doesn't fail, so I wonder
    // anchor doesn't know how to serialize private enum fields.
    // anyway will set these to public and will verify later.
    pub pointer: u8,
    pub queue: Vec<CommitBid>,
}

impl CommitQueue {
    const MAX_CAPACITY: usize = 10;
    pub const LEN: usize = DISCRIMINATOR
        + BUMP
        + PUBKEY_BYTES
        + BYTE
        + (UNSIGNED_128 + (CommitBid::LEN * Self::MAX_CAPACITY));

    pub fn initialize(&mut self, bump: u8, session: Pubkey) {
        self.bump = bump;
        self.session = session;

        self.pointer = 0;
        self.queue = Vec::new();

        // emit event
    }

    // algo: add until full, when full insert at index and remove last in queue
    pub fn insert(&mut self, commit: Commit, sealed_bid_by_index: &Account<SealedBidByIndex>) {
        let mut index = self.queue.len();

        while index != 0 && commit.amount > self.queue[index - 1].amount {
            index -= 1;
        }

        if index == self.queue.len() && index != Self::MAX_CAPACITY {
            self.queue.push(CommitBid {
                owner: sealed_bid_by_index.key(),
                bid_index: sealed_bid_by_index.bid_index,
                amount: commit.amount,
                commit_leader_board_index: sealed_bid_by_index.commit_leader_board_index,
            });

        // emit event element was added
        } else if index != Self::MAX_CAPACITY {
            self.queue.insert(
                index,
                CommitBid {
                    owner: sealed_bid_by_index.key(),
                    bid_index: sealed_bid_by_index.bid_index,
                    amount: commit.amount,
                    commit_leader_board_index: sealed_bid_by_index.commit_leader_board_index,
                },
            );

            // emit event element was added
        }
    }

    pub fn remove(&mut self) -> Option<CommitBid> {
        if !(self.queue.len() > Self::MAX_CAPACITY) {
            return None;
        }

        // log transfer refund
        return self.queue.pop();
    }

    pub fn dequeue(&mut self) {
        self.pointer += 1;
    }

    pub fn get(&self) -> CommitBid {
        return self.queue[self.pointer as usize].clone();
    }

    pub fn current(&self) -> u8 {
        return self.pointer + 1;
    }

    pub fn is_valid_insert(
        &self,
        commit_leader_board: &Account<CommitLeaderBoard>,
        sealed_bid_by_index: &Account<SealedBidByIndex>,
    ) -> bool {
        let commit = commit_leader_board.pool.list
            [sealed_bid_by_index.commit_leader_board_index as usize]
            .clone()
            .unwrap();

        return !(self.queue.len() != Self::MAX_CAPACITY
            || (self.queue.len() == Self::MAX_CAPACITY
                && commit.position.amount > self.queue[self.queue.len() - 1].amount));
    }

    pub fn is_valid_open_bid(&self, owner: Pubkey) -> bool {
        return !(self.queue[self.pointer as usize].clone().owner == owner);
    }

    pub fn is_valid_dequeue(&self) -> bool {
        return self.pointer < Self::MAX_CAPACITY as u8;
    }

    pub fn is_valid_session(&self, session: Pubkey) -> bool {
        return !(self.session == session);
    }

    // not sure what this is here for...
    pub fn add(&mut self) {}
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct Commit {
    pub bid_index: u32,
    pub amount: u64,
}

impl Commit {
    const LEN: usize = UNSIGNED_32 + UNSIGNED_64;
}

// QueuedCommit / CommitBid
#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct CommitBid {
    pub owner: Pubkey,
    pub bid_index: u32,

    // should this be called value?
    pub amount: u64,
    pub commit_leader_board_index: u32,
}

impl CommitBid {
    pub const LEN: usize = PUBKEY_BYTES + UNSIGNED_32 + UNSIGNED_64 + UNSIGNED_32;
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub enum SealedBidRoundStatus {
    Enqueue,
    SealedBidPhase,
    UnsealBidPhase,
    Closed,
    Canceled,
}

impl SealedBidRoundStatus {
    pub const LEN: usize = BYTE;
}

// sealed bid system
//  SealedBidRound
//  SealedBidByIndex
//  CommitLeaderBoard
//  CommitQueue
//  SealedBidTokenStakeAccount
//  CommitTokenAccount
