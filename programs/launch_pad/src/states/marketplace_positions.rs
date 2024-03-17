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
