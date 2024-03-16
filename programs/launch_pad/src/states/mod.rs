pub mod indexer_status;
pub mod program_authority;
// pub mod round_status;
pub mod session;
pub mod session_indexer;

pub use indexer_status::*;
pub use program_authority::*;
// pub use round_status::*;
pub use session::*;
pub use session_indexer::*;

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
//              TokenStakingAccount
//              [..] ::SealedBidder::
//                  :SealedBid
//                  :TokenCommitStaking --  don't think it should be coupled with sealedBid with multiple instances,
//                                          no benefit because of leaderBoard, and it's more convienent to have a single account
//              CommitRankLeaderBoard
//          ::TickBid::
//              [n]RoundStatus / TickBidRound / SealedBidConfig
//              TickBidLeaderBoard
//              MarketplacePositions
//          VestedConfig / VestedStatus /VestedState
//              [..] ::VestedAccount::
//                  :VestedAccountByOwner
//                  :VestedAccountByPagination
//      ProgramAuthority
//          SessionIndexer
//          ActiveSessionIndex
//          EnqueueSessionIndex
//          TokenStaking
//          MarketplaceMatchers

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
