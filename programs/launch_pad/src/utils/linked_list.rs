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
        + (BYTE + (BYTE + Node::<T>::LEN) * MAX_PARTICPANTS)
        + (BYTE + STACK * MAX_PARTICPANTS);
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

    pub fn insert(&mut self, pos: u32, node: &mut Node<T>) {
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

    pub fn update(&mut self, node: Node<T>) {
        self.list[node.index as usize] = Some(node.clone());
    }

    pub fn remove(&self, pos: u32) -> Node<T> {
        return self.list[pos as usize].clone().unwrap();
    }

    pub fn set_to_none(&mut self, pos: u32) {
        let node = self.remove(pos);
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
        let u32_as_bytes: [u8; 4] = [0, u24_as_bytes[0], u24_as_bytes[1], u24_as_bytes[2]];
        return u32::from_be_bytes(u32_as_bytes);
    }

    pub fn last_element(&self) -> u32 {
        let u24_as_bytes: [u8; 3] = self.stack[self.stack.len() - 1];
        let u32_as_bytes: [u8; 4] = [0, u24_as_bytes[0], u24_as_bytes[1], u24_as_bytes[2]];
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
