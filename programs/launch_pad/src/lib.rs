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

    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        instructions::mint_tokens::handler(ctx, amount)
    }

    pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
        instructions::transfer_tokens::handler(ctx, amount)
    }

    pub fn add_bid_token_mint(ctx: Context<AddBidTokenMint>) -> Result<()> {
        instructions::add_bid_token_mint::handler(ctx)
    }

    pub fn create_session(ctx: Context<CreateSession>, input: SessionParams) -> Result<()> {
        instructions::session::handler(ctx, input)
    }

    pub fn create_session_sealed_bid_round(
        ctx: Context<CreateSessionSealedBidRound>,
    ) -> Result<()> {
        instructions::sealed_bid_round::handler(ctx)
    }

    pub fn create_commit_leader_board(ctx: Context<CreateCommitLeaderBoard>) -> Result<()> {
        instructions::commit_leader_board::handler(ctx)
    }

    pub fn reallocate_commit_leader_board(ctx: Context<ReallocateCommitLeaderBoard>) -> Result<()> {
        instructions::reallocate_commit_leader_board::handler(ctx)
    }

    pub fn create_session_commit_queue(ctx: Context<CreateSessionCommitQueue>) -> Result<()> {
        instructions::commit_queue::handler(ctx)
    }

    pub fn create_commit_bid_vault(ctx: Context<CreateCommitBidVault>) -> Result<()> {
        instructions::commit_bid_vault::handler(ctx)
    }

    pub fn create_token_stake_vault(ctx: Context<CreateTokenStakeVault>) -> Result<()> {
        instructions::token_stake_vault::handler(ctx)
    }

    pub fn create_tick_bid_round(ctx: Context<CreateSessionTickBidRound>) -> Result<()> {
        instructions::tick_bid_round::handler(ctx)
    }

    pub fn create_venture_token_escrow(ctx: Context<CreateVentureTokenEscrow>) -> Result<()> {
        instructions::venture_token_escrow::handler(ctx)
    }

    pub fn create_vested_token_escrow(ctx: Context<CreateVestedTokenEscrow>) -> Result<()> {
        instructions::vested_token_escrow::handler(ctx)
    }

    pub fn create_vested_config(ctx: Context<CreateVestedConfig>) -> Result<()> {
        instructions::vested_config::handler(ctx)
    }

    pub fn submit_sealed_bid(ctx: Context<SubmitSealedBid>, commit_hash: Pubkey) -> Result<()> {
        instructions::submit_sealed_bid::handler(ctx, commit_hash)
    }

    pub fn submit_unsealed_bid(
        ctx: Context<SubmitUnsealedBid>,
        amount: u64,
        index: u32,
        _secret: [u8; 32],
    ) -> Result<()> {
        instructions::submit_unsealed_bid::handler(ctx, amount, index)
    }

    pub fn submit_commit_bid(ctx: Context<CommitBidBySession>) -> Result<()> {
        instructions::submit_commit_bid::handler(ctx)
    }

    pub fn session_registration(ctx: Context<SessionRegistration>) -> Result<()> {
        instructions::session_registration::handler(ctx)
    }

    pub fn open_bid(ctx: Context<OpenBid>) -> Result<()> {
        instructions::open_bid::handler(ctx)
    }

    pub fn execute_bid(ctx: Context<ExecuteBid>) -> Result<()> {
        instructions::execute_bid::handler(ctx)
    }

    pub fn transfer_rent_zero_copy(ctx: Context<TransferRentZeroCopy>) -> Result<()> {
        instructions::transfer_rent_zero_copy::handler(ctx)
    }

    pub fn assign_zero_copy(ctx: Context<AssignZeroCopy>) -> Result<()> {
        instructions::assign_zero_copy::handler(ctx)
    }

    pub fn realloc_zero_copy(ctx: Context<ReallocZeroCopy>) -> Result<()> {
        instructions::realloc_zero_copy::handler(ctx)
    }

    pub fn initialize_zero_copy(ctx: Context<InitializeZeroCopy>) -> Result<()> {
        instructions::initialize_zero_copy::handler(ctx)
    }

    pub fn update_leader_board(ctx: Context<UpdateLeaderBaord>, src: u32, dest: u32) -> Result<()> {
        instructions::update_leader_board::handler(ctx, src, dest)
    }
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
