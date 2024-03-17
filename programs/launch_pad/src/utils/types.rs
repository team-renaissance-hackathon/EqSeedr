use super::constants::*;
use anchor_lang::prelude::*;

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct Indexer {
    year_timestamp: i64,
    year: u16,
    week: u8,
    nonce: u8,
    delta_index: u8,
}

impl Indexer {
    pub const LEN: usize = SIGNED_64 + UNSIGNED_16 + UNSIGNED_8 + UNSIGNED_8 + UNSIGNED_8;

    // will work up to year 2038, this needs an update when a new standard exist.
    const YEAR_DELTA: [i64; 4] = [31_622_400, 31_536_000, 31_536_000, 31_536_000];
    const WEEK_DELTA: i64 = 604_800;

    const GENISUS_TIMESTAMP: i64 = 1_704_067_200;
    const INIT_YEAR: u16 = 2024;

    pub fn init(&mut self) {
        self.year_timestamp = Indexer::GENISUS_TIMESTAMP;
        self.year = Indexer::INIT_YEAR;
        self.week = 0;
        self.nonce = 0;
        self.delta_index = 0;
    }

    pub fn update(&mut self) -> Result<()> {
        let clock = Clock::get()?;
        let index = self.delta_index as usize;
        let mut delta = clock.unix_timestamp - self.year_timestamp;

        if delta >= Indexer::YEAR_DELTA[index] {
            self.year_timestamp += Indexer::YEAR_DELTA[index];
            self.year += 1;
            self.delta_index = (self.delta_index + 1) % Indexer::YEAR_DELTA.len() as u8;

            delta = 0;
        }

        if self.week != (delta / Indexer::WEEK_DELTA) as u8 + 1 {
            self.nonce = 0;
        }

        self.week = (delta / Indexer::WEEK_DELTA) as u8 + 1;
        self.nonce += 1;
        return Ok(());
    }
}

// LINKED LIST TYPE
#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct LinkedList<T> {
    pub total: u32,
    head: u32,
    tail: u32,
    list: Vec<Option<Node<T>>>,
    stack: Vec<[u8; 3]>,
}

const STACK: usize = 3;
impl<T: Len + Copy> LinkedList<T> {
    pub const LEN: usize = UNSIGNED_32
        + UNSIGNED_32
        + UNSIGNED_32
        // not sure if this is correct?
        + ((BYTE + T::LEN) * MAX_PARTICPANTS)
        + (STACK * MAX_PARTICPANTS);
    // next
    // prev
    // add
    // remove
    // swap
    // search -> client side

    pub fn next(&self, node: Node<T>) -> Option<Node<T>> {
        match node.next {
            Some(pos) => self.list[pos as usize].clone(),
            None => None,
        }
    }

    pub fn prev(&self, node: Node<T>) -> Option<Node<T>> {
        match node.prev {
            Some(pos) => self.list[pos as usize].clone(),
            None => None,
        }
    }

    fn insert(&mut self, pos: u32, node: &mut Node<T>) {
        if pos == self.head {
            let next_node = self.list[self.head as usize].clone().unwrap();

            self.head = node.index;
            node.prev = None;
            node.next = Some(next_node.index);
        } else if pos >= self.total || self.list[pos as usize].is_none() {
            let prev_node = self.list[self.tail as usize].clone().unwrap();

            self.tail = node.index;
            node.prev = Some(prev_node.index);
            node.next = None;
        } else {
            let next_node = &mut self.list[pos as usize].clone().unwrap().clone();
            let prev_node = &mut self.list[next_node.prev.clone().unwrap() as usize]
                .clone()
                .unwrap();

            prev_node.next = Some(node.index);
            next_node.prev = Some(node.index);
            node.prev = Some(prev_node.index);
            node.next = Some(next_node.index);
        }
    }

    pub fn remove(&self, pos: u32) -> Node<T> {
        return self.list[pos as usize].clone().unwrap();
    }

    pub fn set_to_none(&mut self, pos: u32) {
        let node = self.list[pos as usize].clone().unwrap();
        self.push(node.index);
        self.list[node.index as usize] = None;
    }

    pub fn add(&mut self, pos: u32, node: &mut Node<T>) {
        if !self.stack.is_empty() {
            node.index = self.pop();
            self.list[node.index as usize] = Some(node.clone());
            self.insert(pos, node);
        } else {
            node.index = self.total;
            self.list.push(Some(node.clone()));
            self.insert(pos, node);
            self.total += 1;
        }
    }

    pub fn swap(&mut self, current_pos: u32, new_pos: u32) {
        let node = &mut self.remove(current_pos);
        self.insert(new_pos, node);
    }

    pub fn push(&mut self, pos: u32) {
        let u32_as_bytes: [u8; 4] = pos.to_be_bytes();
        let u24_as_bytes: [u8; 3] = [u32_as_bytes[1], u32_as_bytes[2], u32_as_bytes[3]];
        self.stack.push(u24_as_bytes);
    }

    pub fn pop(&mut self) -> u32 {
        let u24_as_bytes: [u8; 3] = self.stack.pop().unwrap();
        let u32_as_bytes: [u8; 4] = [0, u24_as_bytes[1], u24_as_bytes[2], u24_as_bytes[3]];
        return u32::from_be_bytes(u32_as_bytes);
    }

    pub fn last_element(&self) -> u32 {
        let u24_as_bytes: [u8; 3] = self.stack[self.stack.len() - 1];
        let u32_as_bytes: [u8; 4] = [0, u24_as_bytes[1], u24_as_bytes[2], u24_as_bytes[3]];
        return u32::from_be_bytes(u32_as_bytes);
    }
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct Node<T> {
    index: u32,
    prev: Option<u32>,
    next: Option<u32>,
    position: T,
}

impl<T: Copy> Clone for Node<T> {
    fn clone(&self) -> Self {
        Node {
            index: self.index,
            prev: self.prev,
            next: self.next,
            position: self.position,
        }
    }
}

impl<T: Len> Node<T> {
    pub const LEN: usize = UNSIGNED_32 + (BYTE + UNSIGNED_32) + (BYTE + UNSIGNED_32) + T::LEN;
}

pub trait Len {
    const LEN: usize;
}
