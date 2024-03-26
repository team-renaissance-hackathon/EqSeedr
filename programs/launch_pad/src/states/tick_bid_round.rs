use crate::states::Session;
use crate::utils::*;
use anchor_lang::prelude::*;

// whats the right name?
// round status, round header, round details
#[account]
pub struct TickBidRound {
    pub bump: u8,
    // identifier
    pub index: u8,

    // configuration
    pub token_allocation: u64,
    // can increase on dilution event -> target
    pub ticket_allocation: u64,

    // computed values:
    // user token share distribution -> user ticket amount / ticket_allocation
    // user token amount distribution -> token_allocation * token share distribution

    // status
    status: Status,

    // ticket ticket status
    pub total: u64,
    pub bonus_pool: u64,
    // start scaler at 64
    pub scaler: u64,
    pub avg_tick_depth: u64,

    // total amount of USDC | SOL raised
    // total_amount_bids
    pub bid_sum: u64,

    // total amount tickets issued
    pub total_tickets: u64,

    // total of bids executed
    pub number_of_bids: u32,

    // bid status
    pub last_market_bid: u64,
    pub last_tick_bid: u64,
    pub last_bid_timestamp: i64,
    pub last_bid_slot: u64,
}

impl TickBidRound {
    pub const LEN: usize = DISCRIMINATOR
        + UNSIGNED_8
        + UNSIGNED_8
        + UNSIGNED_64
        + UNSIGNED_64
        + Status::LEN
        + UNSIGNED_64
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

    const MIN: i64 = 60;
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
        self.index = session.next_round();
        self.token_allocation = session.allocate_tokens();
        self.status = Status::Enqueue;

        self.scaler = 64;

        self.avg_tick_depth = 0;
        self.last_market_bid = 0;
        self.last_tick_bid = 0;
        self.last_bid_timestamp = 0;
        self.last_bid_slot = 0;
        self.total = 0;
        self.bonus_pool = 0;

        msg!("round: {}", self.index)
    }

    // OpenRoundStatus
    fn open_round(&mut self) -> Result<()> {
        self.status = Status::Open;
        Ok(())
    }

    // CloseRoundStatus
    fn close_round(&mut self) -> Result<()> {
        self.status = Status::Closed;

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

    // OpenBid - 0
    fn open_bid(&mut self, bid: u64, current: Clock) -> Result<()> {
        // need update average something
        // are these the same???? most resolve
        self.last_market_bid = bid;
        // is this the tick depth?
        // self.round.last_tick_bid = bid;

        self.last_bid_timestamp = current.unix_timestamp;
        self.last_bid_slot = current.slot;
        self.total += 1;
        // log event

        Ok(())
    }

    // ExecuteBid - 0
    fn can_bid_delta(&self, current: Clock) -> bool {
        let delta = current.slot - self.last_bid_slot;
        // log message can't bid because of delta variance
        return !(delta < 10);
    }

    // ExecuteBid - 1
    fn get_current_bid(&self) -> Result<(u64, u64)> {
        let clock = Clock::get()?;
        let targert_delta = self.last_bid_timestamp - clock.unix_timestamp;
        let mut delta = TickBidRound::MIN * 2 as i64;
        let mut tick_depth: u64 = 0;

        while targert_delta > delta {
            delta += delta + TickBidRound::MIN;
            tick_depth += TickBidRound::UNIT;
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
    fn update_bid_status(&mut self, bid: u64, tick: u64, clock: Clock) {
        self.last_market_bid = bid;
        self.last_tick_bid = tick;
        self.last_bid_timestamp = clock.unix_timestamp;
        self.last_bid_slot = clock.slot;

        // log bid, tick, timestamp, and slot
    }

    // where the bulk of the algorthm will be
    // ExecuteBid - 4 | Openbid?
    fn update_ticket_status(&mut self, bid: u64, tick_depth: u64) {
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

        self.avg_tick_depth = (self.avg_tick_depth + tick_depth) / self.total;

        self.bonus_pool += sum;
        self.total += sum + 1;
        // log data
    }

    // ExecuteBid - 6 | OpenBid
    fn transfer() {
        // transfer USDC into session funding account
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
enum Status {
    Enqueue,
    Open,
    Closed,
}

impl Status {
    const LEN: usize = 1;
}

// LEADER BOARD ACCOUNT:
#[account]
pub struct TickBidLeaderBoard {
    pub bump: u8,
    pub session: Pubkey,
    pub pool: TickBidLeaderBoardLinkedList,
}

impl TickBidLeaderBoard {
    pub const LEN: usize = BYTE + PUBKEY_BYTES + TickBidLeaderBoardLinkedList::LEN;

    pub fn initialize(&mut self, bump: u8, session: Pubkey) {
        self.bump = bump;
        self.session = session;

        self.pool = TickBidLeaderBoardLinkedList::new();
    }

    pub fn add(&mut self) {}
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct TickBidLeaderBoardLinkedList {
    pub total: u32,
    head: u32,
    tail: u32,
    list: Vec<TickBidNode>,
    stack: Vec<[u8; 3]>,
}

impl TickBidLeaderBoardLinkedList {
    pub const LEN: usize = UNSIGNED_32
        + UNSIGNED_32
        + UNSIGNED_32
        + (UNSIGNED_128 + TickBidNode::LEN)
        + (UNSIGNED_128 + (UNSIGNED_8 * 3));

    pub fn new() -> Self {
        TickBidLeaderBoardLinkedList {
            total: 0,
            head: 0,
            tail: 0,
            list: Vec::<TickBidNode>::new(),
            stack: Vec::<[u8; 3]>::new(),
        }
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct TickBidNode {
    index: u32,
    prev: Option<u32>,
    next: Option<u32>,
    position: TickBidPosition,
}

impl TickBidNode {
    pub const LEN: usize =
        UNSIGNED_32 + (BYTE + UNSIGNED_32) + (BYTE + UNSIGNED_32) + TickBidPosition::LEN;

    pub fn new() -> Self {
        TickBidNode {
            index: 0,
            prev: None,
            next: None,
            position: TickBidPosition {
                vested_index: 0,
                vested_amount: 0,
            },
        }
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct TickBidPosition {
    pub vested_index: u32,
    pub vested_amount: u64,
}

impl TickBidPosition {
    pub const LEN: usize = UNSIGNED_32 + UNSIGNED_64;
}
