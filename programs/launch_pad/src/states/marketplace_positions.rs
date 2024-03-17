pub struct MarketplacePositions {
    pub pool: LinkedList<MarketPosition>,
}

impl MarketplacePositions {
    // not sure if this is correct
    const Len: usize = DISCRIMINATOR + LinkedList::LEN;

    // add position
    //  - insert index
    //  - insert position
    //  - balance
    //  - fee payout
    //  - bid target
    //  -- update VestedAccountByOwner to isMarketMaker = true

    // update position
    //  - current index
    //  - current position
    //  - insert position
    //  - balance?
    //  - fee payout?
    //  - bid target?

    // update position
    //  - current index
    //  -- update VestedAccountByOwner to isMarketMaker = false
}

// LINKED LIST TYPE
pub struct LinkedList<T> {
    pub total: u32,
    head: u32,
    tail: u32,
    pool: Vec<Option<Node<T>>>,
    stack: Vec<[u8; 3]>,
}

impl LinkedList<T> {
    const LEN: usize = UNSIGNED_32 + UNSIGNED_32 + UNSIGNED_32 + ((BYTE + Node<T>::LEN) * MAX) + (STACK * MAX);
    const MAX: usize = 65000;
    const STACK: usize = 3;
    // next
    // prev
    // add
    // remove
    // swap
    // search -> client side

    fn next(&self, node: Node<T>) -> Option<Node<T>> {
        match node.next {
            Some(pos) => self.pool[pos],
            None => None,
        }
    }

    fn prev(&self, node: Node<T>) -> Option<Node<T>> {
        match node.prev {
            Some(pos) => self.pool[pos],
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
        } else if (!self.stack.is_empty() && self.pool[pos] == None) || pos >= self.total {
            let tail_node = self.pool[self.tail as usize].unwrap();

            return node.position.is_greater_than_or_eqal(tail_node.position);
        } else {
            let next_node = self.pool[pos as usize].unwrap();
            let prev_node = self.pool[next_node.prev.unwrap() as usize].unwrap();

            return node.position.is_less_than(head_node.position)
                && node.position.is_greater_than_or_eqal(prev_node.position);
        }
    }

    fn insert(&self, pos: u64, node: Node<T>) {
        if pos == self.head.index {
            let next_node = self.pool[self.head as usize].unwrap();

            self.head = node.index;
            node.prev = none;
            node.next = Some(next_node.index);
        } else if pos >= self.total || self.pool[pos] == None {
            let prev_node = self.pool[self.tail as usize].unwrap();

            self.tail = node.index;
            node.prev = Some(prev_node.index);
            node.next = None;
        } else {
            let next_node = self.pool[pos as usize].unwrap();
            let prev_node = next_node.prev.unwrap();

            prev_node.next = Some(node.index);
            next_node.prev = Some(node.index);
            node.prev = Some(prev_node.index);
            node.next = Some(next_node.index);
        }
    }

    fn remove(&self, pos: usize) -> node {
        return self.pool[pos].unwrap();
    }

    fn set_to_none(&self, pos: usize) {
        let node = self.remove(pos);
        self.push(node.index);
        self.pool[node.index as usize] = None;
    }

    fn add(&self, pos: usize, node: Node<T>) {
        if !self.stack.is_empty() {
            node.index = self.pop();
            self.pool[node.index as usize] = Some(node);
            self.insert(pos, node);
        } else {
            node.index = self.total;
            self.pool.push(Some(node));
            self.insert(pos, node);
            self.total += 1;
        }
    }

    fn swap(&self, current_pos: u64, new_pos: u64) {
        let node = self.remove(current_pos);
        self.insert(new_pos, node);
    }

    fn push(index: u32) {
        let u32_as_bytes: [u8; 4] = pos.to_be_bytes();
        self.stack.push(u32_as_bytes[1..]);
    }

    fn pop() -> u32 {
        let u32_as_bytes: [u8; 4] = [0, ..self.stack.pop()];
        return u32::from_be_bytes(u32_as_bytes);
    }

    fn last_element() -> u32 {
        let u32_as_bytes: [u8; 4] = [0, ..self.stack[self.stack.len() - 1]];
        return u32::from_be_bytes(u32_as_bytes);
    }
}

pub struct Node<T> {
    index: u32,
    prev: Option<u32>,
    next: Option<u32>,
    position: T,
}

impl Node<T> {
    // have a trait bound so that T has -> LEN
    const LEN: usize = UNSIGNED_32 + (BYTE + UNSIGNED_32) + (BYTE + UNSIGNED_32) + T::LEN;
}

// MARKET MAKER
pub struct MarketPosition {
    pub index: u32,
    pub vested_index: u32,
    pub bid_amount: u64,
    pub bid_target: u64,
    pub fee_pay: u64,
}

impl MarketPosition {
    const Len: usize = UNSIGNED_32 + UNSIGNED_32 + UNSIGNED_64 + UNSIGNED_64 + UNSIGNED_64;

    // wrap into a trait?
    fn is_less_than(&self, position: MarketPosition) -> bool {
        if !(self.bid_target < position.bid_target) {
            return false;
        }

        return true;
    }

    fn is_greater_than_or_eqal(&self, position: MarketPosition) -> bool {
        if !(self.bid_target >= position.bid_target) {
            return false;
        }

        return true;
    }
}

// MARKET MATCHER
pub struct MarketMatcher {
    pub address: Pubkey,
    pub is_active_status: bool,
    pub locked_amount: u64,
}

pub struct MarketMatchPool {
    pub current_market_taker: Pubkey,

    // is sorted list
    pub market_taker_pool: Vec<MarketTaker>,
    // small fee to add to list
    // fee dynamically increases when lots of market takers are in pool
    // because it is a scarce resource
}

impl MarketMatchPool {
    // add market taker
    //      add token to stake
    // update status
    //      adds / removes to and from pool in sorted order
    //      updates the current market taker, if conditions are right
    // remove market taker
    //      remove token from stake

    // fall back logic for MatchBid
    //      current market taker
    //      2 min window colloct fee
    //      5 min window not risk losing reward
    //      5 min window next market taker to execute bid
    //      10 min window open for market maker next in queue to execute bid
    //      open for all to make bid, place bid, earn ticket -> should pay? or is free?
}
