use anchor_lang::prelude::*;

#[error_code]
pub enum ProgramError {
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
}
