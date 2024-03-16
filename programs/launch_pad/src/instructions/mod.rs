pub mod create_session;
pub mod initialize;
// pub mod create_rounds;
// pub mod create_session_marketplace;

// how do I resolve this? other than using a different name for handler?
pub use create_session::*;
pub use initialize::*;
// pub use create_rounds::*;
// pub use create_session_marketplace::*;

// flow:
// init bid
// ticket total: 1
// average depth: 0

// VALIDATIONS
//      Initialize
//          Authority
//      CreateSession
//          Authority
//          ProgramAuthority

// TICK-BID
//  BIDDER/INVESTOR
//      Register
//          Bidder
//          NewVestedAccountByOwner
//          NewVestedAccountByIndex
//          tickBidLeaderBoard
//          ValidSession
//      OpenBid
//          Bidder -> Signer / Payer / Authority
//              - mut
//          ValidVestedAccountByOwner
//              - mut
//              - ownder == bidder      @ ProgramError::InvalidVestedOwner
//          CommitQueue
//              - mut
//              - bidder == nextBidder  @ ProgramError::NotCurrentCommitBidder
//          ValidTickBidRound
//              - mut
//              - valid_tick_bid_round.status == open        @ ProgramError::InvalidOpenBid
//              - valid_tick_bid_round.total == 0            @ ProgramError::InvalidOpenBid
//              - valid_tick_bid_round.indexer == valid_session.indexer
//          ValidSession
//              - mut
//              - status == open        @ ProgramError::SessionNotOpenStatus
//      ExecuteBid
//          Bidder -> Signer / Payer / Authority
//              - mut
//          ValidVestedAccountByOwner
//              - mut
//              - ownder == bidder      @ ProgramError::InvalidVestedOwner
//          ValidTickBidRound
//              - mut
//              - status == open
//              - total != ticket_allocation
//              - valid_tick_bid_round.indexer == valid_session.indexer
//          ValidSession
//              - mut
//      UpdateLeaderBoard -> postInstruction
//          INPUT:
//              vested_account_current_pos
//              vested_account_next_pos
//          Payer -> Signer
//          VestedAccountByOwner
//          TickBidLeaderBoard

//  MARKETPLACE
//      MARKET MAKER
//          AddPosition
//              INPUT:
//                  transfer/delegate? -> balance_amount
//                  target_bid
//                  fee_payout
//              MarketMaker -> Signer / Authority
//              ValidVestedAccountByOwner -> has_one = authority
//              MarketplacePosition
//          UpdatePosition
//              INPUT:
//                  ?balance_amount
//                  ?target_bid
//                  ?fee_payout
//              MarketMaker -> Signer / Authority
//              ValidVestedAccountByOwner -> has_one = authority
//              MarketplacePosition
//          RemovePosition
//              MarketMaker -> Signer / Authority
//              MarketplacePosition

//      MARKET MATCHER -> I think is missing something, will look into it later
//          MatchBid
//              ValidMarketMatcher -> Signer
//              ValidMarketMakerPosition
//              MarketplacePosition
//              ValidTickBidRound
//              ValidSession
//          AddToPool
//              INPUT:
//                  staking_amount
//                  valid_position
//              MarketMatcher -> Signer / Authority
//              MarketplaceMatchers
//              MatcherTokenAccount
//              TokenStakingAccount
//              TokenMint
//              TokenProgram
//          UpdateToPool
//              INPUT:
//                  staking_amount
//                  current_postion
//                  valid_position
//              MarketMatcher -> Signer / Authority
//              MarketplaceMatchers
//              TokenAccount
//              MatcherTokenAccount
//              TokenStakingAccount
//              TokenMint
//              TokenProgram
//          RemoveFromPool
//              INPUT:
//                  current_position
//              MarketMatcher -> Signer / Authority
//              MarketplaceMatchers
//              MatcherTokenAccount
//              TokenStakingAccount
//              TokenMint
//              TokenProgram
//          SetActiveStatus
//              INPUT:
//                  current_position -> if adding position is max number
//                  valid_position
//              MarketMatcher -> Signer / Authority
//              MatcherPool
//              MarketplaceMatchers

//  MISC
//      OpenRoundStatus
//          Payer
//              - mut
//          ValidTickBidRound
//              - mut
//              - status == Enqueue
//              - index == session.current_round
//          Session
//      CloseRoundStatus
//          Payer
//              - mut
//          ValidTickBidRound
//              - mut
//              - status == open
//              - valid_tick_bid_round.total == session.ticket_allocation
//          Session
//              - mut

// SEALED-BID
//  BIDDER/INVESTOR
//      SubmitSealedBid
//          INPUT:
//              commit_hash
//          Bidder -> Signer / Payer / Authority
//          SealedBid
//          SealedBidRound
//          ValidSession
//          TokenStakingSessionAccount
//          BidderTokenAccount
//          TokenProgram
//      UnsealBid
//          INPUT:
//              bid_amount
//              secret
//          Bidder -> Signer / Payer / Authority
//          SealedBid
//          SealedBidRound
//          ValidSession
//          CommitLeaderBoard
//      CommitBid
//          Bidder -> Signer / Payer / Authority
//          CommitQueue
//          CommitLeaderBoard
//          ValidSession

// LAUNCH-PAD-SESSION
//  PROJECT DEVELOPER
//      CreateSession
//          Authority
//          SessionIndexer
//          NewSession
//          EnqueueIndexer
//          SystemProgram
//      CreateSessionMarketplace
//          Authority
//          Session
//          NewSessionMarketplace
//          SystemProgram
//      CreateTickBidRound
//          Authority
//          Session
//          NewTickBidRound
//          SystemProgram
//      CreateSealBidRound
//          Authority
//          Session
//          NewSealBidRound
//          SystemProgram
//      CreateCommitLeaderBoard
//          Authority
//          Session
//          NewCommitLeaderBoard
//          SystemProgram
//      CreateCommitQueue
//          Authority
//          Session
//          NewCommitQueue
//          SystemProgram
//      CreateTickBidLeader
//          Authority
//          Session
//          NewTickBidLeader
//          SystemProgram
//      CreateVestingEscrowAccount
//          Authority
//          Session
//          NewVestingEscrowAccount
//          SystemProgram
//      CreateVestingConfig
//          Authority
//          Session
//          NewVestingConfig
//          SystemProgram
//      CreateSessionTokenStaking
//          Authority
//          Session
//          NewSessionTokenStaking
//          SystemProgram

//      RemoveSession
//          Authority
//          Session
//          EnqueueIndexer
//          SystemProgram
//      RemoveSessionMarketplace
//          Authority
//          SessionMarketplace
//          SystemProgram
//      RemoveTickBidRound
//          Authority
//          TickBidRound
//          SystemProgram
//      RemoveSealBidRound
//          Authority
//          SealBidRound
//          SystemProgram
//      RemoveCommitLeaderBoard
//          Authority
//          CommitLeaderBoard
//          SystemProgram
//      RemoveCommitQueue
//          Authority
//          CommitQueue
//          SystemProgram
//      RemoveTickBidLeader
//          Authority
//          TickBidLeader
//          SystemProgram
//      RemoveVestingEscrowAccount
//          Authority
//          VestingEscrowAccount
//          SystemProgram
//      RemoveVestingConfig
//          Authority
//          VestingConfig
//          SystemProgram
//      RemoveSessionTokenStaking
//          Authority
//          SessionTokenStaking
//          SystemProgram

// VESTING
//  INVESTOR
//      ClaimLockedTokens
//          Authority
//          VestedAccountByOwner
//          Session
//          TickBidRound
//          TokenAccount
//          TokenEscrowAccount
//          TokenProgram
//          SystemProgram

// ACCOUNT INITIALIZATIONS
