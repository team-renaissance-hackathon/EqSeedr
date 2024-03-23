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

    // currently not testing since would need a
    // transaction to set this value.
    pub fn is_valid_sealed_bid_phase(&self) -> bool {
        match self.status {
            Status::SealedBidPhase => !true,
            _ => !false,
        }
    }

    // currently not testing since would need a
    // transaction to set this value.
    // should consider the constraint by messure of time
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

    pub fn create_node(&self, bidder_index: u32, amount: u64) -> CommitNode {
        CommitNode {
            index: self.pool.total,
            prev: None,
            next: None,
            position: Commit {
                bidder_index,
                amount,
            },
        }
    }
    pub fn add(&mut self, node: &mut CommitNode, index: u32) {
        self.pool.add(index, node);
    }

    pub fn is_valid_session(&self, session: Pubkey) -> bool {
        return !(self.session == session);
    }

    pub fn is_valid_node(&self, pos: u32, amount: u64) -> bool {
        self.pool.node_is_valid(pos, amount)
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

    fn next(&self, node: CommitNode) -> Option<CommitNode> {
        match node.next {
            Some(pos) => self.list[pos as usize].clone(),
            None => None,
        }
    }

    fn prev(&self, node: CommitNode) -> Option<CommitNode> {
        match node.prev {
            Some(pos) => self.list[pos as usize].clone(),
            None => None,
        }
    }

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
    Closed,
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
//  SealedBidTokenStakeAccount
//  CommitTokenAccount
