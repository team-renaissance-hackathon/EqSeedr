use crate::states::Session;
use crate::utils::*;
use anchor_lang::prelude::*;

// whats the right name?
// round status, round header, round details
#[account]
pub struct TickBidRound {
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
    pub status: Status,

    // ticket ticket status
    pub total: u64,
    pub bonus_pool: u64,
    // start scaler at 64
    pub scaler: u64,
    pub average_tick_depth: u64,

    // bid status
    pub last_market_bid: u64,
    pub last_tick_bid: u64,
    pub last_bid_timestamp: i64,
    pub last_bid_slot: u128,
}

impl TickBidRound {
    pub const LEN: usize = DISCRIMINATOR
        + UNSIGNED_8
        + UNSIGNED_64
        + UNSIGNED_64
        + Status::LEN
        + UNSIGNED_64
        + UNSIGNED_64
        + UNSIGNED_64
        + SIGNED_64
        + UNSIGNED_128;

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
    pub fn initialize(&mut self, session: Session) {
        self.round.index = session.round;
        // self.round.token_allocation = session.token_allocation / session.target_rounds;
        // self.round.ticket_allocation = session.ticket_allocation / session.target_rounds;
        self.round.token_allocation = session.allocate_tokens();
        self.round.ticket_allocation = session.allocate_tickets();
        self.round.status = Status::Enqueue;
        self.round.scaler = 64;
        self.round.average_tick_depth = 0;
        self.round.last_market_bid = 0;
        self.round.last_tick_bid = 0;
        self.round.last_bid_timestamp = 0;
        self.round.last_bid_slot = 0;
        self.round.total = 0;
        self.round.bonus_pool = 0;
    }

    // OpenRoundStatus
    fn open_round(&self) -> Result<()> {
        self.round.status = Status::Open;
        Ok(())
    }

    // CloseRoundStatus
    fn close_round(&self) -> Result<()> {
        self.status == Status::Closed;

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
    fn open_bid(&self, bid: u64, current: Clock) -> Result<()> {
        // need update average something
        // are these the same???? most resolve
        self.round.last_market_bid = bid;
        // is this the tick depth?
        // self.round.last_tick_bid = bid;

        self.round.last_bid_timestamp = current.unix_timestamp;
        self.round.last_bid_slot = current.slot;
        self.round.total += 1;
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
    fn get_current_bid(self) -> Result<(u64, u64)> {
        let clock = Clock.get()?;
        let targert_delta = last_bid_timestamp - clock.unix_timestamp;
        let delta = MIN * 2 as i64;
        let tick_depth: u64 = 0;

        while targert_delta > delta {
            delta += delta + MIN;
            tick_depth += UNIT;
        }

        let bid = if tick_depth > BYTES_8 || last_market_bid >> tick_depth == IS_ZERO {
            UNIT
        } else if tick_depth <= UNIT {
            let price = (MULTIPLIER >> tick_depth) * self.last_market_bid;
            price + (price * tick_depth / PERCENT_100)
        } else {
            let reduce = (UNIT << (tick_depth - OFFSET));
            let price = self.last_market_bid / reduce;
            price + (price * tick_depth / PERCENT_100)
        };

        Ok((bid, tick_depth))
    }

    // ExecuteBid - 2
    fn can_bid_queue(&self, bid: u64, queue: MarketMakerQueue) -> bool {
        // log message if can't bid, can't bid, because queue is filled
        // name candidate?
        // top bid, next bid, highest bid
        return queue.next_bid >= bid;
    }

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
        let sum: u64;
        for num in RoundStatus::FIB_SEQUENCE[..tick_depth] {
            sum += num / self.ticket_bag_scaler;
        }

        if tick > self.average_tick_depth {
            self.ticket_bag_scaler -= 1;
        }

        self.average_tick_depth = (self.average_tick_depth + tick_depth) / self.total;

        self.bonus_pool + sum;
        self.total += sum + 1;
        // log data
    }

    // ExecuteBid - 6 | OpenBid
    fn transfer() {
        // transfer USDC into session funding account
    }
}

pub enum Status {
    Enqueue,
    Open,
    Closed,
}

impl Status {
    const LEN: usize = 1;
}
