pub struct Rank {
    pub leading_round: u8,
    pub index: u8,
    pub highest_overall_bid: [BidRankNode; 10],
    pub nearest_avg_bid: [BidRankNode; 10],
    pub reward_distribution_highest_bid: u64,
    pub reward_distribution_avg_bid: u64,
}

pub struct BidRankNode {
    vested_index: u32,
    value: u64,
}

pub struct RewardPayment {}
