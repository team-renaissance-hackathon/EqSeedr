// small fee to add to list
// fee dynamically increases when lots of market takers are in pool
// because it is a scarce resource
// will require zero loading
use crate::utils::*;
use anchor_lang::prelude::*;

#[account]
pub struct MarketplaceMatchers {
    pub bump: u8,
    pub authority: Pubkey,
    // active list -> sorted -> linked list
    // need an advance schedular state / logic
    // current market taker
    // need to set a size limit on pool, stack, active_list
    pub active_pool: MarketplaceMatchersLinkedList,
    pub current_market_matcher: u32,
    pub pool: Vec<u32>, // index map to market matcher, should include the pubkey?
    pub stack: Vec<u32>,
}

impl MarketplaceMatchers {
    pub const LEN: usize = 1000;
    pub fn initialize(&mut self, bump: u8, authority: Pubkey) {
        self.bump = bump;
        self.authority = authority;

        self.active_pool = MarketplaceMatchersLinkedList::new();

        // plan to use a schedular instead of using this.
        self.current_market_matcher = 0;

        // what is this? don't remember.
        self.pool = Vec::<u32>::new();
        self.stack = Vec::<u32>::new();
    }

    pub fn add() {

        // default state

        // add to pool
        // transfer
    }
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

    // fn set_status(pos: u32, is_active: bool, node: MarketMatcherPositionNode) {
    //     if !is_active {
    //         self.active_list.add(pos, node);
    //     } else {
    //         self.active_list.remove(pos, node);
    //     }
    // }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct MarketplaceMatchersLinkedList {
    total: u32,
    head: u32,
    tail: u32,
    list: Vec<MarketMatcherPositionNode>,
    stack: Vec<[u8; 3]>,
}

impl MarketplaceMatchersLinkedList {
    pub const LEN: usize = UNSIGNED_32
        + UNSIGNED_32
        + UNSIGNED_32
        + (UNSIGNED_64 + MarketMatcherPositionNode::LEN)
        + (UNSIGNED_64 + BYTE * 3);

    pub fn new() -> Self {
        MarketplaceMatchersLinkedList {
            total: 0,
            head: 0,
            tail: 0,
            list: Vec::<MarketMatcherPositionNode>::new(),
            stack: Vec::<[u8; 3]>::new(),
        }
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct MarketMatcherPositionNode {
    index: u32,
    prev: Option<u32>,
    next: Option<u32>,
    position: MarketMatcherPosition,
}

impl MarketMatcherPositionNode {
    pub const LEN: usize =
        UNSIGNED_32 + (BYTE + UNSIGNED_32) + (BYTE + UNSIGNED_32) + MarketMatcherPosition::LEN;
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct MarketMatcherPosition {
    index: u32,
    staked: u64,
    // other stat for schedular
}

impl MarketMatcherPosition {
    const LEN: usize = UNSIGNED_32 + UNSIGNED_64;
}

// MARKET MATCHER
// seeds
//      - authority pubkey
//      - b"market-matcher"
#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct MarketMatcher {
    pub index: Option<u32>,
    pub authority: Pubkey,
    pub is_active: bool,
    pub locked_amount: u64,
    pub balance: u64,
}

impl MarketMatcher {
    pub const LEN: usize = 8 + 32 + 1 + 8 + 8;
    // fn init(&self, params: Params, authority: Pubkey) {
    //     self.index = params.index;
    //     self.authority = authority;
    //     self.is_active = false;
    //     self.locked_amount = params.amount;
    //     // created_date?
    //     // should there be a cool down for staking and unstaking?
    //     // cool down when setting is_active?
    // }

    // fn update(&self, amount_delta: u64) {
    //     // has to be active status false in order to update
    //     // has to be in pool
    //     // increase staking -> can only increase
    //     self.locked_amount += amount_delta;
    // }

    pub fn remove(&mut self) {
        // has tobe active status false
        // remove from pool
        // index = None
        // transfer -> from staking to account

        self.locked_amount = 0;
        self.index = None;
    }

    // fn set_status(&self) {
    //     self.is_active = !self.is_active;
    // }

    pub fn collect(&self) {
        // has to be active status false
        // transfer all balance
    }
}

// MatchBid -> is heavy in algorithmic logic so will handle that last
// AddtoPool
// UpdateToPool
// RemoveFromPool
// SetActiveStatus -> will interact with LinkedList
//      - data -> node
//      - MarketMatcher::set_status()
//      - MarketplaceMatchers::set_status(pos, is_active, node)
