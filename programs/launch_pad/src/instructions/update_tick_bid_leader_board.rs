#[derive(Accounts)]
pub struct UpdateTickBidLeaderBoard<'info> {
    // may need to use lookup tables
    pub payer: Signer<'info>,
    pub session: Account<'info, Session>,
    pub vested_account_by_owner: Account<'info, VestedAccountByOwner>,

    // session
    pub tick_bid_leader_board_by_overall: Account<'info, TickBidLeaderBoard>,
    pub tick_bid_leader_board_by_last: Account<'info, TickBidLeaderBoard>,
    pub tick_bid_leader_board_by_avg: Account<'info, TickBidLeaderBoard>,
    // linkedlist | queue ?
    // pub tick_bid_leader_board_by_total: Account<'info, TickBidLeaderBoard>,
    pub tick_bid_leader_board_by_limit: Account<'info, LimitLeaderBoard>,

    // round
    pub tick_bid_round_leader_board_by_overall: Account<'info, TickBidLeaderBoard>,
    pub tick_bid_round_leader_board_by_last: Account<'info, TickBidLeaderBoard>,
    pub tick_bid_round_leader_board_by_avg: Account<'info, TickBidLeaderBoard>,
    // linkedlist | queue ?
    // pub tick_bid_round_leader_board_by_total: Account<'info, TickBidLeaderBoard>,
    pub tick_bid_round_leader_board_by_limit: Account<'info, LimitLeaderBoard>,
    // buffer accont, to be cleared, which is set during execution of bid
}

//  Post instruction
//  update leader baord
//  vested account by index
//      - tick bid leader board index   -> if new, this value will get added
//                                      -> this value will be used for validation
// NOTES:
//      right now this is only taking into consideration the tick bid leader board for
//      the session, but not for the specified round because there is none for the rounds
//      but that looks like that will change since it might be needed since thinking about it more
//      but I am still giving it more thought. but most likely will be a thing.

// UPDATE STATE::
//  tick bid leader board
//

// ranky types
//  - highest overall bid
//  - highest last bid
//  - highest avg. bid / cost basis
//  - most number of bids
//  - lowest overall bid
//  - lowest avg. bid / cost basis
//  - the higest tick depth
//  - highest bid sum

//  - overall bid   -> highest - lowest     -> linked list
//  - last bid      -> highest - lowest     -> linked list
//  - avg. bid      -> highest - lowest     -> linked list
//  - total bid     -> highest - lowest     -> linked list | queue -> limit 5 ?
//  - higest tick depth                     -> queue -> limit 5
//  - most number of tick bids              -> queue -> limit 5

//  - pointer to bidder that is closest to the avg market value
//      - overall bid
//      - last bid
//      - avg. bid

// system to enforce that the leader board gets updated
//  - buffer state is set to the bidder address when executing bid
//  - buffer must be cleared when updating leader board
//  - if buffer is not cleared, then if next bidder executes bid,
//  - it will be at the current tick, as if the last bid never happen
//  - in such a case, the funds dont get refunded as penalty.
