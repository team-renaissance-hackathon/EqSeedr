pub struct CompleteRound {
    // session
    // current round
    // signer
    // account that tracks the winners
    // leadear_board
}

pub fn handler() {
    // rank.leading_round
    let round_index = rank.index as usize;
    rank.index += 1;

    tick_bid_round.highest_overall_bid;
    tick_bid_round.highest_overall_bid_by_vested_index;

    tick_bid_round.nearest_avg_bid;
    let node = leadear_board.read(tick_bid_round.nearest_avg_bid_by_leadear_board_index);
    node.vested_index;

    let pool = tick_bid_round.bonus_pool;
    session.bonus_pool = pool >> 1 + pool % 2;

    vesting_account_by_owner.round_status[round_index].total_tokens += pool >> 1;
    vested_account_by_owner.session_status.total_tokens += pool >> 1;
}
