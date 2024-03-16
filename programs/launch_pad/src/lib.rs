use anchor_lang::prelude::*;
pub mod instructions;
pub mod states;
pub mod utils;
pub use instructions::*;

declare_id!("7GKWqKvkev22SYs2HEb1jw6h4uHJwLVKpEcxVUqTZKxG");

#[program]
pub mod launch_pad {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }

    pub fn create_session(ctx: Context<CreateSession>, input: SessionParams) -> Result<()> {
        instructions::create_session::handler(ctx, input)
    }
}

// TICK-BID
//  BIDDER/INVESTOR
//      ExecuteBid
//  MARKETPLACE
//      MARKET MAKER
//          AddPosition
//          UpdatePosition
//          RemovePosition
//      MARKET TAKER
//          MatchBid
//          AddToPool
//          UpdateToPoll
//          RemoveFromPool
//          SetActiveStatus
//  MISC
//      UpdateLeaderBaord -> postInstruction
//      Register        -> preInstruction | isolatedInstruction
//      OpenBidStatus   -> preInstruction | isolatedInstruction
//      CloseBidStatus  -> postInstruction | isolatedInstruction

// SEALED-BID
//  BIDDER/INVESTOR
//      SubmitSealedBid
//      RevealSealedBid -> add to leaderboard
//      CommitBid
//  MISC
//      UpdateLeaderBaord -> postInstruction -> not sure if it should be an postInstru
//      OpenBidStatus   -> preInstruction | isolatedInstruction
//      CloseBidStatus  -> postInstruction | isolatedInstruction

// LAUNCH-PAD-SESSION
//  PROJECT DEVELOPER
//      {LaunchProject}
//          CreateSession
//          CreateSessionMarketplace
//          CreateRounds
//          CreateSealedBid
//          CreateSealedBidLeaderBaord
//          CreateTickBidLeaderBoard
//          CreateVestingEscrowAccount
//      {CancelProject}
//          RemoveSession
//          RemoveSessionMarketplace
//          RemoveRounds
//          RemoveSealedBid
//          RemoveLeaderBoard
//          RemoveSealedBidLeaderBaord
//          RemoveTickBidLeaderBoard
//          RemoveVestingEscrowAccount
//  MISC
//      OpenBidSession      -> preInstruction | isolatedInstruction
//      CloseBidSession     -> postInstruction | isolatedInstruction
//  PROCESS
//      MINTING
//          10% PlatformTokens
//          % SessonRoundTokens
//

// INITIALIZE PROGRAM
//  ProgramAuthority
//  Indexer

// VESTING
//  INVESTOR
//      ClaimLockedTokens
//      StakeTokens
//      UnstakeTokens
//      AddLockedTokensAsLiquidity
//      RemoveLockedTokensFromLiquidity
//  DEX
//      -- need lock up what type of DEX exist and functionality

// NOTES:
//  use program hooks for the algorithms -> CPI CALLS
//  can exchange algorithms wihtout updating core program
