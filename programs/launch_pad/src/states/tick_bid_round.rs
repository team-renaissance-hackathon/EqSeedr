use crate::states::Session;
use crate::utils::*;
use anchor_lang::prelude::*;

// whats the right name?
// round status, round header, round details
#[account]
pub struct TickBidRound {
    // VALIDATION STATE
    pub bump: u8,
    pub session: Pubkey,
    // identifier
    pub index: u8,

    // configuration
    pub token_allocation: u64,

    // computed values:
    // user token share distribution -> user ticket amount / ticket_allocation
    // user token amount distribution -> token_allocation * token share distribution

    // status
    pub status: TickBidRoundStatus,

    // ticket ticket status
    pub bonus_pool: u64,
    // start scaler at 64
    pub scaler: u64,
    pub avg_tick_depth: u64,

    // total amount of USDC | SOL raised
    // total_amount_bids
    pub bid_sum: u64,
    pub total_tokens: u64,

    // total of bids executed
    pub number_of_bids: u32,

    // bid status
    pub init_market_bid: u64,
    pub last_market_bid: u64,

    // the tick depth of the last bid
    pub last_tick_bid_depth: u64,
    pub last_bid_timestamp: i64,
    pub last_bid_slot: u64,
}

impl TickBidRound {
    pub const LEN: usize = DISCRIMINATOR
        + UNSIGNED_8
        + PUBKEY_BYTES
        + UNSIGNED_8
        + UNSIGNED_64
        + TickBidRoundStatus::LEN
        + UNSIGNED_64
        + UNSIGNED_64
        + UNSIGNED_64
        + UNSIGNED_64
        + UNSIGNED_64
        + UNSIGNED_32
        + UNSIGNED_64
        + UNSIGNED_64
        + SIGNED_64
        + UNSIGNED_64;

    const MIN: i64 = 60; // MINUTE
    const UNIT: u64 = 0b1;
    const OFFSET: u64 = 0b1;
    const PERCENT_100: u64 = 100;
    const IS_ZERO: u64 = 0;
    const MULTIPLIER: u64 = 0b10;
    const BYTES_8: u64 = 0b1000000;
    const FIB_SEQUENCE: [u64; 24] = [
        0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1_597, 2_584, 4_181,
        6_765, 10_946, 17_711, 28_657,
    ];

    // CreateTickBidRound
    pub fn initialize(&mut self, bump: u8, session: &Account<'_, Session>) {
        self.bump = bump;
        self.session = session.key().clone();
        self.index = session.next_round();
        self.token_allocation = session.allocate_tokens();
        self.status = TickBidRoundStatus::Enqueue;

        self.scaler = 64;

        self.avg_tick_depth = 0;
        self.last_market_bid = 0;
        self.last_tick_bid_depth = 0;
        self.last_bid_timestamp = 0;
        self.last_bid_slot = 0;
        self.total_tokens = 0;
        self.bonus_pool = 0;

        msg!("round: {}", self.index)
    }

    pub fn open_bid(&mut self, clock: &Clock, market_bid: u64) {
        self.status = TickBidRoundStatus::Open;
        self.number_of_bids += 1;

        self.init_market_bid = market_bid;
        self.last_market_bid = market_bid;

        self.last_tick_bid_depth = 0;

        self.bid_sum += market_bid;
        self.total_tokens += 1;

        // tick algo will be based on these.
        self.last_bid_timestamp = clock.unix_timestamp;
        self.last_bid_slot = clock.slot;

        // log event
        msg!(
            "{}", // EVENTS:
            "EVENTS",
        )
    }

    pub fn get_index(&self) -> u8 {
        return self.index - 1;
    }

    // CloseRoundStatus
    pub fn close_round(&mut self) -> Result<()> {
        self.status = TickBidRoundStatus::Closed;

        // redistribute bag
        // update round bag
        // update session bag
        // log event

        Ok(())
    }

    // bid execution order::
    // commit_queue.update() -> OpenBid
    // can_bid_delta()
    // execute_bid()
    // can_bid_queue()
    // update bid status
    // update ticket status
    // vested_account_by_owner.update() -> OpenBid | ExecuteBid
    // transfer()

    // ExecuteBid - 1
    pub fn get_current_bid(&self) -> Result<(u64, u64)> {
        let clock = Clock::get()?;
        let target_delta = clock.unix_timestamp - self.last_bid_timestamp;
        let mut delta = TickBidRound::MIN * 2 as i64;
        let mut tick_depth: u64 = 0;

        msg!("A::DEBUG: TARGET DELTA: {}", target_delta);
        msg!("A::DEBUG: MIN DELTA: {}", delta);

        // let mut scale = 1;
        while target_delta > delta {
            tick_depth += TickBidRound::UNIT;
            delta += TickBidRound::MIN * 2 + TickBidRound::MIN * tick_depth as i64;
        }

        let bid = if tick_depth > TickBidRound::BYTES_8
            || self.last_market_bid >> tick_depth == TickBidRound::IS_ZERO
        {
            TickBidRound::UNIT
        } else if tick_depth <= TickBidRound::UNIT {
            let price = (TickBidRound::MULTIPLIER >> tick_depth) * self.last_market_bid;
            price + (price * tick_depth / TickBidRound::PERCENT_100)
        } else {
            let reduce = TickBidRound::UNIT << (tick_depth - TickBidRound::OFFSET);
            let price = self.last_market_bid / reduce;
            price + (price * tick_depth / TickBidRound::PERCENT_100)
        };

        msg!("B::DEBUG: TARGET DELTA: {}", target_delta);
        msg!("B::DEBUG: MIN DELTA: {}", delta);

        Ok((bid, tick_depth))
    }

    // ExecuteBid - 2
    // fn can_bid_queue(&self, bid: u64, queue: MarketMakerQueue) -> bool {
    //     // log message if can't bid, can't bid, because queue is filled
    //     // name candidate?
    //     // top bid, next bid, highest bid
    //     return queue.next_bid >= bid;
    // }

    // ExecuteBid - 3 | OpenBid?
    pub fn update_bid_status(&mut self, market_bid: u64, tick_depth: u64, clock: &Clock) {
        self.last_market_bid = market_bid;
        self.last_tick_bid_depth = tick_depth;
        self.last_bid_timestamp = clock.unix_timestamp;
        self.last_bid_slot = clock.slot;

        msg!(
            "{}{}, {}{}, {}{}, {}{},",
            // log bid, tick depth, timestamp, and slot
            "Last Market Bid: ",
            self.last_market_bid,
            "Last Tick Depth: ",
            self.last_tick_bid_depth,
            "TimeStamp: ",
            self.last_bid_timestamp,
            "Slot: ",
            self.last_bid_slot,
        )
    }

    // where the bulk of the algorthm will be
    // ExecuteBid - 4 | Openbid?
    // &mut self, bid: u64, tick_depth: u64
    pub fn update_pool_status(&mut self, tick_depth: u64) {
        let mut sum: u64 = 0;
        let mut tick = 0;

        while tick != tick_depth {
            let num = TickBidRound::FIB_SEQUENCE[tick as usize];
            sum += num / self.scaler;
            tick += 1;
        }

        if tick_depth > self.avg_tick_depth {
            self.scaler -= 1;
        }

        self.avg_tick_depth = (self.avg_tick_depth + tick_depth) / self.total_tokens;

        self.bonus_pool += sum;
        self.total_tokens += sum + 1;
        // log data
    }

    // :: VALIDATIONS ::
    pub fn is_valid_session(&self, session: Pubkey) -> bool {
        // need to add session
        return self.session == session;
    }

    pub fn is_valid_tick_bid_round(&self, round: u8) -> bool {
        // index -> round index... needs better name reference.
        return self.index == round;
    }

    pub fn is_valid_enqueue_status(&self) -> bool {
        match self.status {
            TickBidRoundStatus::Enqueue => true,
            _ => false,
        }
    }

    pub fn is_valid_open_status(&self) -> bool {
        match self.status {
            TickBidRoundStatus::Open => true,
            _ => false,
        }
    }

    // ExecuteBid - 0 -> :: VALIDATION ::
    pub fn is_valid_delta(&self) -> bool {
        let clock = Clock::get().unwrap();
        let delta = clock.slot - self.last_bid_slot;
        return delta > 10;
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, PartialEq)]
pub enum TickBidRoundStatus {
    Enqueue,
    Open,
    Closed,
}

impl TickBidRoundStatus {
    pub const LEN: usize = UNSIGNED_64 + BYTE;
}

// LEADER BOARD ACCOUNT:
// #[account]
// pub struct TickBidLeaderBoard {
//     pub bump: u8,
//     pub session: Pubkey,
//     pub pool: TickBidLeaderBoardLinkedList,
// }

// impl TickBidLeaderBoard {
//     pub const LEN: usize = BYTE + PUBKEY_BYTES + TickBidLeaderBoardLinkedList::LEN;

//     pub fn initialize(&mut self, bump: u8, session: Pubkey) {
//         self.bump = bump;
//         self.session = session;

//         self.pool = TickBidLeaderBoardLinkedList::new();
//     }

//     pub fn add(&mut self) {}
// }

// #[derive(AnchorDeserialize, AnchorSerialize, Clone)]
// pub struct TickBidLeaderBoardLinkedList {
//     pub total: u32,
//     head: u32,
//     tail: u32,
//     list: Vec<TickBidNode>,
//     stack: Vec<[u8; 3]>,
// }

// impl TickBidLeaderBoardLinkedList {
//     pub const LEN: usize = UNSIGNED_32
//         + UNSIGNED_32
//         + UNSIGNED_32
//         + (UNSIGNED_128 + TickBidNode::LEN)
//         + (UNSIGNED_128 + (UNSIGNED_8 * 3));

//     pub fn new() -> Self {
//         TickBidLeaderBoardLinkedList {
//             total: 0,
//             head: 0,
//             tail: 0,
//             list: Vec::<TickBidNode>::new(),
//             stack: Vec::<[u8; 3]>::new(),
//         }
//     }
// }

// #[derive(AnchorDeserialize, AnchorSerialize, Clone)]
// pub struct TickBidNode {
//     index: u32,
//     prev: Option<u32>,
//     next: Option<u32>,
//     position: TickBidPosition,
// }

// impl TickBidNode {
//     pub const LEN: usize =
//         UNSIGNED_32 + (BYTE + UNSIGNED_32) + (BYTE + UNSIGNED_32) + TickBidPosition::LEN;

//     pub fn new() -> Self {
//         TickBidNode {
//             index: 0,
//             prev: None,
//             next: None,
//             position: TickBidPosition {
//                 vested_index: 0,
//                 vested_amount: 0,
//             },
//         }
//     }
// }

// #[derive(AnchorDeserialize, AnchorSerialize, Clone)]
// pub struct TickBidPosition {
//     pub vested_index: u32,
//     pub vested_amount: u64,
// }

// impl TickBidPosition {
//     pub const LEN: usize = UNSIGNED_32 + UNSIGNED_64;
// }
