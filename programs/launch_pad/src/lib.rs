pub mod experiment;
pub mod instructions;
pub mod states;
pub mod utils;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{allocate, assign, transfer, Allocate, Assign, Transfer};
pub use experiment::*;
pub use instructions::*;

declare_id!("7GKWqKvkev22SYs2HEb1jw6h4uHJwLVKpEcxVUqTZKxG");

#[program]
pub mod launch_pad {
    use super::*;

    pub fn allocate_zero_copy(ctx: Context<AllocateZeroCopy>, amount: u64) -> Result<()> {
        let AllocateZeroCopy {
            payer,
            new_account,
            system_program,
        } = ctx.accounts;

        let space = amount;

        let (_, new_account_bump_seed) =
            Pubkey::find_program_address(&[b"leader-board", payer.key.as_ref()], &ctx.program_id);

        let seeds = &[
            b"leader-board",
            payer.to_account_info().key.as_ref(),
            &[new_account_bump_seed],
        ];

        let signer_seeds = &[&seeds[..]];

        allocate(
            CpiContext::new_with_signer(
                system_program.to_account_info(),
                Allocate {
                    account_to_allocate: new_account.to_account_info(),
                },
                signer_seeds,
            ),
            space,
        )?;

        assign(
            CpiContext::new_with_signer(
                system_program.to_account_info(),
                Assign {
                    account_to_assign: new_account.to_account_info(),
                },
                signer_seeds,
            ),
            &ctx.program_id,
        )?;

        Ok(())
    }

    pub fn transfer_rent_zero_copy(ctx: Context<TransferRentZeroCopy>, amount: u64) -> Result<()> {
        let TransferRentZeroCopy {
            payer,
            new_account,
            system_program,
        } = ctx.accounts;

        let space = amount;

        let rent = Rent::get()?.minimum_balance(space.try_into().expect("overflow"));

        let (_, new_account_bump_seed) =
            Pubkey::find_program_address(&[b"leader-board", payer.key.as_ref()], &ctx.program_id);

        let seeds = &[
            b"leader-board",
            payer.to_account_info().key.as_ref(),
            &[new_account_bump_seed],
        ];

        let signer_seeds = &[&seeds[..]];

        transfer(
            CpiContext::new_with_signer(
                system_program.to_account_info(),
                Transfer {
                    from: payer.to_account_info(),
                    to: new_account.to_account_info(),
                },
                signer_seeds,
            ),
            rent,
        )?;
        Ok(())
    }

    pub fn reallocate_zero_copy(ctx: Context<ReallocateZeroCopy>, amount: u64) -> Result<()> {
        let ReallocateZeroCopy { new_account } = ctx.accounts;

        let space = amount;

        new_account.realloc(space as usize, false)?;

        Ok(())
    }

    pub fn assign_zero_copy(ctx: Context<AssignZeroCopy>) -> Result<()> {
        let AssignZeroCopy {
            payer,
            new_account,
            system_program,
        } = ctx.accounts;

        let (_, new_account_bump_seed) =
            Pubkey::find_program_address(&[b"leader-board", payer.key.as_ref()], &ctx.program_id);

        let seeds = &[
            b"leader-board",
            payer.to_account_info().key.as_ref(),
            &[new_account_bump_seed],
        ];

        let signer_seeds = &[&seeds[..]];

        assign(
            CpiContext::new_with_signer(
                system_program.to_account_info(),
                Assign {
                    account_to_assign: new_account.to_account_info(),
                },
                signer_seeds,
            ),
            &ctx.program_id,
        )?;

        Ok(())
    }

    pub fn initialize_zero_copy(ctx: Context<InitializeZeroCopy>, input: u64) -> Result<()> {
        let new_account = &mut ctx.accounts.new_account.load_init()?;
        // new_account.data[0] = input;

        msg!("Initialize data to: {}", input);
        Ok(())
    }

    pub fn update_zero_copy(ctx: Context<UpdateZeroCopy>, input: u64) -> Result<()> {
        let existing_account = &mut ctx.accounts.existing_account.load_mut()?;
        // existing_account.data[1] = input;

        msg!("Updated data to: {}", input);
        Ok(())
    }

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }

    pub fn add_bid_token_mint(ctx: Context<AddBidTokenMint>) -> Result<()> {
        instructions::add_bid_token_mint::handler(ctx)
    }

    pub fn create_session(ctx: Context<CreateSession>, input: SessionParams) -> Result<()> {
        instructions::create_instance::session::handler(ctx, input)
    }

    pub fn create_session_sealed_bid_round(
        ctx: Context<CreateSessionSealedBidRound>,
    ) -> Result<()> {
        instructions::create_instance::sealed_bid_round::handler(ctx)
    }

    pub fn create_commit_leader_board(ctx: Context<CreateCommitLeaderBoard>) -> Result<()> {
        instructions::create_instance::commit_leader_board::handler(ctx)
    }

    pub fn reallocate_commit_leader_board(ctx: Context<ReallocateCommitLeaderBoard>) -> Result<()> {
        instructions::create_instance::reallocate_commit_leader_board::handler(ctx)
    }

    pub fn create_session_commit_queue(ctx: Context<CreateSessionCommitQueue>) -> Result<()> {
        instructions::create_instance::commit_queue::handler(ctx)
    }

    pub fn create_commit_bid_vault(ctx: Context<CreateCommitBidVault>) -> Result<()> {
        instructions::create_instance::commit_bid_vault::handler(ctx)
    }

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

    pub fn create_vested_token_escrow(ctx: Context<CreateVestedTokenEscrow>) -> Result<()> {
        instructions::create_instance::vested_token_escrow::handler(ctx)
    }

    pub fn create_vested_config(ctx: Context<CreateVestedConfig>) -> Result<()> {
        instructions::create_instance::vested_config::handler(ctx)
    }

    // pub fn create_session_marketplace(
    //     ctx: Context<CreateSessionMarketplacePositions>,
    // ) -> Result<()> {
    //     instructions::create_session_marketplace::handler(ctx)
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

    pub fn session_registration(ctx: Context<SessionRegistration>) -> Result<()> {
        instructions::session_registration::handler(ctx)
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
