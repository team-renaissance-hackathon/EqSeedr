pub mod indexer_status;
pub mod marketplace_matchers;
pub mod marketplace_positions;
pub mod program_authority;
pub mod sealed_bid_round;
pub mod session;
pub mod session_indexer;
pub mod tick_bid_round;

pub use indexer_status::*;
pub use marketplace_matchers::*;
pub use marketplace_positions::*;
pub use program_authority::*;
pub use sealed_bid_round::*;
pub use session::*;
pub use session_indexer::*;
pub use tick_bid_round::*;

// STATE
//      Indexer
//      SessionStatus
//      SealedBidStatus
//
//      RoundStatus
//      Marketplace
//      TickBidLeaderBoard
//      SealedBidLeaderBoard
//      VestedConfig / State
//      VestedAccount
//      VestedAccountIndex

// TYPES
//      Indexer

// how to structure multiple instances and pagenation of accounts
// STRUCTURE
//      [..]SessionStatus / Session
//          ::EscrowAccounts::
//              TokenEscrowAccount
//              FundingEscrowAccount
//          SealedStatus / SealedBidRound / SealedBidConfig
//              TokenStakingSession
//              [..] ::SealedBidder::
//                  :SealedBid
//                  :TokenCommitStaking --  don't think it should be coupled with sealedBid with multiple instances,
//                                          no benefit because of leaderBoard, and it's more convienent to have a single account
//              CommitRankLeaderBoard
//              CommitQueue
//          ::TickBid::
//              [n]RoundStatus / TickBidRound / SealedBidConfig
//              TickBidLeaderBoard
//              MarketplacePositions
//          VestedConfig / VestedStatus /VestedState
//              [..] ::VestedAccount::
//                  :VestedAccountByOwner
//                  :VestedAccountByIndex
//      ProgramAuthority
//          SessionIndexer
//          ActiveSessionIndex
//          EnqueueSessionIndex
//          TokenStaking
//          MarketplaceMatchers
//              [..]MarketMatcherByOwner

// SEEDS
// Conslidated view
// expanded view
// relationship / heirarchy view
// structured view
// listed view

// LEGEND:
//  ::LABEL::           - label for related section of accounts
//      ACCOUNT
//  ::LABEL::           - accounts that share a direct one to one relationship
//      :ACCOUNT_A
//      :ACCOUNT_B
//  [n]ACCOUNT          - constant n of instances of accounts
//  [..]ACCOUNT         - multiple instances of accounts
