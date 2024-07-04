use anchor_lang::prelude::{
    borsh::{BorshDeserialize, BorshSerialize},
    *,
};

use crate::{
    states::VestedAccountByOwner,
    utils::{BYTE, UNSIGNED_32, UNSIGNED_64},
};

#[account(zero_copy)]
pub struct LeaderBoard {
    pub session: Pubkey,
    pub bump: u8,
    pub round: u8,
    pub total: [u8; 4],
    pub head: [u8; 4],
    pub tail: [u8; 4],
    pub data: [u8; LeaderBoard::LEN],
}

impl LeaderBoard {
    pub const LEN: usize = 10240 * 2 - (1 + 32 + 1 + 1 + 8 + 8 + 8);
}

impl LeaderBoard {
    pub fn next_index(&self) -> u32 {
        let total = u32::from_be_bytes(self.total);
        let index = total * Node::LEN as u32;
        return index;
    }

    pub fn update(&mut self, node: &Node) -> Result<()> {
        let index = node.index as usize;
        let data = node.try_to_vec()?;
        self.data[index..(index + Node::LEN)].copy_from_slice(&data);
        Ok(())
    }

    pub fn read(&self, index: usize) -> Node {
        let data = &self.data[index..(index + Node::LEN)];
        Node::try_from_slice(data).unwrap()
    }

    pub fn insert(&mut self, dest: u32, node: &mut Node) -> Result<()> {
        let head = u32::from_be_bytes(self.head);
        let tail = u32::from_be_bytes(self.tail);

        if dest == head {
            let mut next_node = self.read(head as usize);

            self.head = node.index.to_be_bytes();
            node.prev = None;
            node.next = Some(next_node.index);
            next_node.prev = Some(node.index);

            self.update(node)?;
            self.update(&next_node)?;
        } else if dest == self.next_index() {
            let mut prev_node = self.read(tail as usize);

            self.tail = node.index.to_be_bytes();
            node.prev = Some(prev_node.index);
            node.next = None;
            prev_node.next = Some(node.index);

            self.update(node)?;
            self.update(&prev_node)?;
        } else {
            let mut next_node = self.read(dest as usize);
            let mut prev_node = self.read(next_node.prev.clone().unwrap() as usize);

            prev_node.next = Some(node.index);
            next_node.prev = Some(node.index);
            node.prev = Some(prev_node.index);
            node.next = Some(next_node.index);

            self.update(node)?;
            self.update(&next_node)?;
            self.update(&prev_node)?;
        }

        Ok(())
    }

    pub fn remove(&mut self, src: u32) -> Result<Node> {
        let node = self.read(src as usize);

        if !node.prev.is_none() && !node.next.is_none() {
            let mut prev_node = self.read(node.prev.unwrap() as usize);
            let mut next_node = self.read(node.next.unwrap() as usize);

            prev_node.next = Some(next_node.index);
            next_node.prev = Some(prev_node.index);

            self.update(&next_node)?;
            self.update(&prev_node)?;
        } else if !node.prev.is_none() {
            let mut prev_node = self.read(node.prev.unwrap() as usize);
            prev_node.next = None;

            self.update(&prev_node)?;
        } else if !node.next.is_none() {
            let mut next_node = self.read(node.next.unwrap() as usize);
            next_node.next = None;

            self.update(&next_node)?;
        }

        Ok(node)
    }

    pub fn swap(&mut self, src: u32, dest: u32, position: Position) -> Result<()> {
        let mut node = self.remove(src)?;
        node.position = position;
        self.insert(dest, &mut node)?;

        Ok(())
    }

    pub fn add(&mut self, dest: u32, position: Position) -> Result<()> {
        let mut total = u32::from_be_bytes(self.total);
        let index = total * Node::LEN as u32;

        let mut node = Node {
            index: index,
            prev: None,
            next: None,
            position: position,
        };

        if index == 0 {
            self.update(&node)?;
        } else {
            self.insert(dest, &mut node)?;
        }

        total += 1;
        self.total = total.to_be_bytes();

        Ok(())
    }

    pub fn is_valid_src(
        leader_board: &AccountLoader<LeaderBoard>,
        src: u32,
        vested_member: &Box<Account<VestedAccountByOwner>>,
    ) -> Result<bool> {
        let leader_board = &mut leader_board.load()?;
        if src != leader_board.next_index() {
            let node = leader_board.read(src as usize);
            return Ok(node.position.vested_index == vested_member.bid_index);
        }

        let round_index = leader_board.round as usize;
        return Ok(!vested_member.round_status[round_index].is_on_leader_board);
    }

    pub fn is_valid_dest(
        leader_board: &AccountLoader<LeaderBoard>,
        dest: u32,
        vested_member: &Box<Account<VestedAccountByOwner>>,
    ) -> Result<bool> {
        let leader_board = &mut leader_board.load()?;
        let round_index = leader_board.round as usize;
        let (_, avg_bid) = vested_member.get_avg_bid_by_round(round_index);

        if leader_board.next_index() == 0 {
            return Ok(true);
        }

        if leader_board.next_index() == dest {
            let tail = u32::from_be_bytes(leader_board.tail);
            let node = leader_board.read(tail as usize);

            return Ok(node.position.avg_bid >= avg_bid);
        }

        if dest == u32::from_be_bytes(leader_board.head) {
            let head = u32::from_be_bytes(leader_board.head);
            let node = leader_board.read(head as usize);

            return Ok(avg_bid > node.position.avg_bid);
        }

        let next_node = leader_board.read(dest as usize);
        let prev_node = leader_board.read(next_node.prev.unwrap() as usize);

        return Ok(prev_node.position.avg_bid >= avg_bid && avg_bid > next_node.position.avg_bid);
    }
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Node {
    pub index: u32,
    pub prev: Option<u32>,
    pub next: Option<u32>,
    pub position: Position,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Position {
    pub vested_index: u32,
    pub avg_bid: u64,
}

impl Node {
    pub const LEN: usize =
        UNSIGNED_32 + (BYTE + UNSIGNED_32) + (BYTE + UNSIGNED_32) + Position::LEN;
}

impl Position {
    pub const LEN: usize = UNSIGNED_32 + UNSIGNED_64;
}
