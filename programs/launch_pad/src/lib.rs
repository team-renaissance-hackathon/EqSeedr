pub mod instructions;
pub mod states;
pub mod utils;
use anchor_lang::prelude::*;
pub use instructions::*;

declare_id!("7GKWqKvkev22SYs2HEb1jw6h4uHJwLVKpEcxVUqTZKxG");

#[program]
pub mod launch_pad {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }

    pub fn create_session(ctx: Context<CreateSession>, input: SessionParams) -> Result<()> {
        instructions::create_instance::session::handler(ctx, input)
    }

    pub fn create_session_sealed_bid_round(
        ctx: Context<CreateSessionSealedBidRound>,
    ) -> Result<()> {
        instructions::create_instance::sealed_bid_round::handler(ctx)
    }

    pub fn create_session_commit_leader_board(
        ctx: Context<CreateSessionCommitLeaderBoard>,
    ) -> Result<()> {
        instructions::create_instance::commit_leader_board::handler(ctx)
    }

    pub fn create_session_commit_queue(ctx: Context<CreateSessionCommitQueue>) -> Result<()> {
        instructions::create_instance::commit_queue::handler(ctx)
    }

    // pub fn create_commit_token_account(ctx: Context<CreateCommitTokenAccount>) -> Result<()> {
    //     instructions::create_commit_token_account::handler(ctx)
    // }

    pub fn create_sealed_bid_token_stake_account(
        ctx: Context<CreateSealedBidTokenStakeAccount>,
    ) -> Result<()> {
        instructions::create_instance::sealed_bid_token_stake_account::handler(ctx)
    }

    pub fn create_tick_bid_round(ctx: Context<CreateSessionTickBidRound>) -> Result<()> {
        instructions::create_instance::tick_bid_round::handler(ctx)
    }

    pub fn create_session_tick_bid_leader_board(
        ctx: Context<CreateSessionTickBidLeaderBoard>,
    ) -> Result<()> {
        instructions::create_instance::tick_bid_leader_board::handler(ctx)
    }

    // pub fn create_session_marketplace(
    //     ctx: Context<CreateSessionMarketplacePositions>,
    // ) -> Result<()> {
    //     instructions::create_session_marketplace::handler(ctx)
    // }

    // pub fn create_vested_config_by_session(
    //     ctx: Context<CreateVestedConfigBySession>,
    // ) -> Result<()> {
    //     instructions::create_vested_config_by_session::handler(ctx)
    // }

    // pub fn submit_sealed_bid(ctx: Context<SubmitSealedBid>, commit_hash: Pubkey) -> Result<()> {
    //     instructions::submit_sealed_bid::handler(ctx, commit_hash)
    // }

    // pub fn submit_unsealed_bid(
    //     ctx: Context<SubmitUnsealedBid>,
    //     amount: u64,
    //     index: u32,
    //     _secret: [u8; 32],
    // ) -> Result<()> {
    //     instructions::submit_unsealed_bid::handler(ctx, amount, index)
    // }

    // pub fn submit_commit_bid(ctx: Context<CommitBidBySession>) -> Result<()> {
    //     instructions::submit_commit_bid::handler(ctx)
    // }

    // pub fn session_registration(ctx: Context<SessionRegistration>) -> Result<()> {
    //     instructions::session_registration::handler(ctx)
    // }
}

// TICK-BID
//  BIDDER/INVESTOR
//      ExecuteBid
//      OpenBid
//  MARKETPLACE
//      MARKET MAKER
//          AddPosition
//          UpdatePosition
//          RemovePosition
//      MARKET MATCHER
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
//          CreateRound
//          CreateSealedBid
//          CreateCommitLeaderBaord
//          CreactCommitQueue
//          CreateTickBidLeaderBoard
//          CreateVestingEscrowAccount
//      {CancelProject}
//          RemoveSession
//          RemoveSessionMarketplace
//          RemoveRounds
//          RemoveSealedBid
//          RemoveLeaderBoard
//          RemoveCommitQueue
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
