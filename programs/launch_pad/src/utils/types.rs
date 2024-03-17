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
#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct LinkedList<T> {
    pub total: u32,
    head: u32,
    tail: u32,
    pool: Vec<Option<Node<T>>>,
    stack: Vec<[u8; 3]>,
}

const STACK: usize = 3;
impl<T: Len> LinkedList<T> {
    const LEN: usize = UNSIGNED_32
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

    fn next(&self, node: Node<T>) -> Option<Node<T>> {
        match node.next {
            Some(pos) => self.pool[pos as usize],
            None => None,
        }
    }

    fn prev(&self, node: Node<T>) -> Option<Node<T>> {
        match node.prev {
            Some(pos) => self.pool[pos as usize],
            None => None,
        }
    }

    fn is_valid(&self, pos: u32, node: Node<T>) -> bool {
        // place in validation check
        //  - add position -> insert
        //          -> pos >= self.total && self.stack.is_empty()
        //                  || pos <= self.total && self.pool[pos] == None && self.last_element() == pos
        //          -> self.is_valid(pos, node)
        //  - remove position -> remove
        //          -> pos < self.total
        //          -> self.pool[pos] != None
        //          -> self.pool[pos].position.index == vested_account_by_owner.index
        //  - update postion -> swap
        //          -> new_pos < self.total, current_pos < self.total,
        //          -> self.pool[new_pos] != None, self.pool[current_pool] != None
        //          -> self.pool[current_pos].position.index == vested_account_by_owner.index
        //          -> self.is_valid(pos, node)

        if self.total == 0 {
            return true;
        }

        if pos == self.head {
            let head_node = self.pool[self.head as usize].unwrap();

            return node.position.is_less_than(head_node.position);
        } else if (!self.stack.is_empty() && self.pool[pos as usize].is_none()) || pos >= self.total
        {
            let tail_node = self.pool[self.tail as usize].unwrap();

            return node.position.is_greater_than_or_equal(tail_node.position);
        } else {
            let next_node = self.pool[pos as usize].unwrap();
            let prev_node = self.pool[next_node.prev.unwrap() as usize].unwrap();

            return node.position.is_less_than(next_node.position)
                && node.position.is_greater_than_or_equal(prev_node.position);
        }
    }

    fn insert(&self, pos: u32, node: &mut Node<T>) {
        if pos == self.head {
            let next_node = self.pool[self.head as usize].unwrap();

            self.head = node.index;
            node.prev = None;
            node.next = Some(next_node.index);
        } else if pos >= self.total || self.pool[pos as usize].is_none() {
            let prev_node = self.pool[self.tail as usize].unwrap();

            self.tail = node.index;
            node.prev = Some(prev_node.index);
            node.next = None;
        } else {
            let next_node = &mut self.pool[pos as usize].unwrap();
            let prev_node = &mut self.pool[next_node.prev.unwrap() as usize].unwrap();

            prev_node.next = Some(node.index);
            next_node.prev = Some(node.index);
            node.prev = Some(prev_node.index);
            node.next = Some(next_node.index);
        }
    }

    fn remove(&self, pos: usize) -> Node<T> {
        return self.pool[pos].unwrap();
    }

    fn set_to_none(&self, pos: usize) {
        let node = self.remove(pos);
        self.push(node.index);
        self.pool[node.index as usize] = None;
    }

    fn add(&self, pos: u32, node: &mut Node<T>) {
        if !self.stack.is_empty() {
            node.index = self.pop();
            self.pool[node.index as usize] = Some(Node {
                index: node.index,
                prev: node.prev,
                next: node.next,
                position: node.position,
            });
            self.insert(pos, node);
        } else {
            node.index = self.total;
            self.pool.push(Some(Node {
                index: node.index,
                prev: node.prev,
                next: node.next,
                position: node.position,
            }));
            self.insert(pos, node);
            self.total += 1;
        }
    }

    fn swap(&self, current_pos: u32, new_pos: u32) {
        let node = &mut self.remove(current_pos as usize);
        self.insert(new_pos, node);
    }

    fn push(&self, pos: u32) {
        let u32_as_bytes: [u8; 4] = pos.to_be_bytes();
        let u24_as_bytes: [u8; 3] = [u32_as_bytes[1], u32_as_bytes[2], u32_as_bytes[3]];
        self.stack.push(u24_as_bytes);
    }

    fn pop(&self) -> u32 {
        let u24_as_bytes: [u8; 3] = self.stack.pop().unwrap();
        let u32_as_bytes: [u8; 4] = [0, u24_as_bytes[1], u24_as_bytes[2], u24_as_bytes[3]];
        return u32::from_be_bytes(u32_as_bytes);
    }

    fn last_element(&self) -> u32 {
        let u24_as_bytes: [u8; 3] = self.stack[self.stack.len() - 1];
        let u32_as_bytes: [u8; 4] = [0, u24_as_bytes[1], u24_as_bytes[2], u24_as_bytes[3]];
        return u32::from_be_bytes(u32_as_bytes);
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct Node<T> {
    index: u32,
    prev: Option<u32>,
    next: Option<u32>,
    position: T,
}

// impl<T: Compare> Node<T> {
//     fn is_less_than(&self, node: Node<T>) -> bool {
//         return self.position.is_less_than(node.position.is_less_than);
//     }
// }

impl<T: Len> Node<T> {
    const LEN: usize = UNSIGNED_32 + (BYTE + UNSIGNED_32) + (BYTE + UNSIGNED_32) + T::LEN;
}

impl<T: Compare> Node<T> {}

trait Len {
    const LEN: usize;
}

trait Compare {
    fn is_less_than() -> bool;
}
