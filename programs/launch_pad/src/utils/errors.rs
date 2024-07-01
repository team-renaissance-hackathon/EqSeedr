use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Exceeds 32 max character limit")]
    InvalidTokenName,

    #[msg("Token Allocation is not Evenally divisible")]
    InvalidTokenAllocation,

    #[msg("Target Rounds not with in Range")]
    InvalidRounds,

    #[msg("Launch Date Delta is invalid")]
    InvalidLaunchDate,

    #[msg("Mint Authority does not Session Creator")]
    ExpectMintAuthorityToCreateSession,

    #[msg("Session can not Exceed Target Rounds")]
    MaxRoundSet,

    #[msg("Sealed Bid Round Already Exist For Session")]
    SessionSealedBidRoundAlreadyExist,

    #[msg("Session Commit Bid Leader Board Already Exist")]
    SessionCommitLeaderBoardAlreadyExist,

    #[msg("Session Tick Bid Rounds, All 10 Rounds Exist")]
    SessionTickBidRoundMaxRoundSet,

    #[msg("Session Tick Bid Leader Board Already Exist")]
    SessionTickBidLeaderBoardAlreadyExist,

    #[msg("Session Marketplace Positions Already Exist")]
    SessionMarketplacePositionsAlreadyExist,

    #[msg("Session Commit Leader Board Max Allocation")]
    SessionCommitLeaderBoardMaxAllocation,

    #[msg("Sealed Bid Round Has Invalid Session")]
    InvalidSealedBidRound,

    #[msg("Commit Bid Vault Already Exist")]
    CommitBidVaultAlreadyExist,

    #[msg("Invalid Bid Token")]
    InvalidBidToken,

    #[msg("Vested Token Escrow Already Exist")]
    VestedTokenEscrowAlreadyExist,

    #[msg("Invalid Token Token")]
    InvalidTokenMint,

    #[msg("Vested Config Already Exist")]
    VestedConfigAlreadyExist,

    #[msg("Invalid Vested Config")]
    InvalidVestedConfig,

    #[msg("Bid Not Commited!")]
    BidNotCommitted,

    #[msg("Bid Already Refunded!")]
    BidIsAlreadyRefunded,

    #[msg("Bid Already Commited!")]
    BidAlreadyCommited,

    #[msg("Invalid Owner Of Sealed Bid By Index!")]
    InvalidOwnerOfSealedBidByIndex,

    #[msg("Stake Is Already Unlocked!")]
    StakeIsAlreadyUnlocked,

    #[msg("Bid Is Not Unsealed!")]
    BidNotUnsealed,

    // this seems incorrect
    #[msg("Invalid Owner Of Sealed Bid By Index!")]
    InvalidSession,

    #[msg("Invalid Tick Bid Round!")]
    InvalidTickBidRound,

    #[msg("Invalid Tick Bid Round Status!")]
    InvalidTickBidRoundStatus,

    #[msg("Is Empty Queue!")]
    IsEmptyQueue,

    #[msg("Invalid Vested Owner!")]
    InvalidVestedOwner,

    #[msg("Session Commit Queue Already Exist!")]
    SessionCommitQueueAlreadyExist,

    #[msg("Invalid Venture Token Mint!")]
    InvalidVentureTokenMint,

    #[msg("Invalid Token Owner!")]
    InvalidTokenOwner,

    #[msg("Invalid Mint Authority!")]
    InvalidMintAuthority,

    #[msg("Invalid Unsealed Bid!")]
    InvalidUnsealedBid,

    #[msg("Invalid Token Stake Vault!")]
    InvalidTokenStakeVault,

    #[msg("Invalid Vested Account Owner!")]
    InvalidVesterOwner,

    #[msg("Not Enough Tokens On Vested Token Escrow")]
    NotEnoughTokensOnVestedEscrow,

    #[msg("Vested Tokens Already Claimed!")]
    VestedTokensAlreadyClaimed,
}
