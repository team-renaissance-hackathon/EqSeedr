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
//              INPUT: -> can sanitize input data? I think we should, the prev, and next, santize to None, sanitize index to last index if exceeds
//                  node
//                      index = insert position
//                      prev = None
//                      next = None
//                      position -> MarketPosition
//                          vested_index
//                          balance_delta
//                          target_bid_delta
//                          fee_payout_delta
//              MarketMaker -> Signer / Authority
//                  - mut
//              ValidVestedAccountByOwner
//                  - mut
//                  - has_one == authority
//                  - market_position == false
//                  - node.position.index == index
//                  -- update VestedAccountByOwner to isMarketMaker = true
//              MarketplacePositions
//                  - mut
//                  - self.pos_is_valid(pos)
//                  - self.node_is_valid(pos, node)
//          UpdatePosition
//              INPUT:
//                  balance_amount_delta
//                  target_bid_delta
//                  fee_payout_delta
//                  current_position
//                  new_position
//              MarketMaker -> Signer / Authority
//                  - mut
//              ValidVestedAccountByOwner
//                  - market_position == true
//                  - has_one == authority
//                  - node.position.index == index
//              MarketplacePositions
//                  - mut
//                          I THINK SOME OF THE VALIDATION HERE IS WRONG, NEED CONFIRM -> need to check the change in delta
//                  - self.node.position.index == vested_account_by_owner.index
//                  - !self.get(new_pos).is_none() && !self.get(current_pos).is_none()
//                  - self.get(current_pos).upwrap().position.index == vested_account_by_owner.index
//                  - self.valid_target_bid_update(target_bid_delta, current_pos, new_pos) -> I think I don't need it
//                  - self.pos_is_valid(current_position) && self.pos_is_valid(new_position)
//                  - self.node_is_valid(new_position, node)
//          RemovePosition
//              INPUT:
//                  position
//              MarketMaker -> Signer / Authority
//                  - mut
//              ValidVestedAccountByOwner
//                  - mut
//                  - market_position == true
//                  - has_one == authority
//                  - node.position.index == index
//                  -- update VestedAccountByOwner to isMarketMaker = false
//              MarketplacePosition
//                  - mut
//                  - self.node.position.index == vested_account_by_owner.index
//                  - position < self.pool.total
//                  - !self.get(pos).is_none()

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
//              Authority
//              MarketMatcher
//                  - init
//              MarketplaceMatchers
//                  - mut
//              MatcherTokenAccount
//              TokenStakingAccount
//              TokenMint
//              TokenProgram
//          UpdateToPool
//              INPUT:
//                  staking_amount
//                  current_postion
//                  valid_position
//              Authority
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
//              Authority
//              MarketMatcher
//              MarketplaceMatchers
//                  - mut
//              MatcherTokenAccount
//              TokenStakingAccount
//              TokenMint
//              TokenProgram
//          SetActiveStatus
//              INPUT:
//                  valid_position
//              Authority
//                  - mut
//              MarketMatcher
//                  - mut
//                  - has_one = authority
//                  - locked_amount != 0
//                  - cool_down_status <= delta or increase fee to set active?
//              MarketplaceMatchers
//                  - mut
//                  - market_matcher.index == self.active_list.index
//                  - self.pos_is_valid(pos)
//                  - self.node_is_valid(pos, node)

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
//      CreateSessionMarketplacePositions
//          Authority
//              - mut
//          NewSessionMarketplacePositions
//              - init
//              - !session.has_marketplace_positions
//          Session
//              - mut
//              - has_one = authority
//          SystemProgram
//      CreateTickBidRound
//          Authority
//              - mut
//          NewTickBidRound
//              - init
//              - seed
//                  - index
//                  - session.key
//                  - b"round-status"
//          Session
//              - mut
//              - has_one = authority
//              - !session.all_tick_bid_rounds_set
//          SystemProgram
//      CreateSealBidRound
//          Authority
//              - mut
//          NewSealBidRound
//              - init
//              - !session.has_sealed_bid_round -> not sure if need this
//          Session
//              - mut
//              - has_one = authority
//          SystemProgram
//      CreateCommitLeaderBoard
//          Authority
//              - mut
//          NewCommitLeaderBoard
//              - init
//              - !session.has_commit_leader_board -> we need the check list, need everything created before rounds can begin
//          Session
//              - mut
//              - has_one = authority
//          SystemProgram
//      CreateCommitQueue
//          Authority
//              - mut
//          NewCommitQueue
//              - init
//              - !session.has_commit_queue
//          Session
//              - mut
//              - has_one = authority
//          SystemProgram
//      CreateSessionTickBidLeaderBoard
//          Authority
//              - mut
//          NewTickBidLeaderBoard
//              - init
//              - !session.has_tick_bid_leader_board
//          Session
//              - mut
//              - has_one = authority
//          SystemProgram
//      CreateVestingEscrowAccount
//          Authority
//              - mut
//          NewVestingEscrowAccount
//              - init
//              - session.has_vesting_escrow_account
//          Session
//              - mut
//              - has_one = authority
//              - session.data.token_mint == token_mint.key
//          TokenMint
//          TokenProgram
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
