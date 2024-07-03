use crate::states::{
    vested_accounts::*, 
    ProgramAuthority, Session};
use crate::utils::errors::ErrorCode;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

#[derive(Accounts)]
pub struct ClaimVested<'info> {
    #[account(mut)]
    pub bidder: Signer<'info>,

    #[account(
        mut,
        constraint = bidder_vested_account.owner == bidder.key()
            @ ErrorCode::InvalidVestedOwner,
    )]
    pub bidder_vested_account: Account<'info, VestedAccountByOwner>,

    #[account(
        mut, //I just put mut initially, but should be init_if_needed if not initialized somewhere
        constraint = bidder_project_token_account.owner == bidder.key()
            @ ErrorCode::InvalidTokenOwner,
    )]
    pub bidder_project_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub vested_token_escrow: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [b"authority"],
        bump = program_authority.bump,
    )]
    pub program_authority: Account<'info, ProgramAuthority>,
   
    pub token_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,

}

pub fn handler(ctx: Context<ClaimVested>, index: u8) -> Result<()> {
    let ClaimVested {
        bidder_vested_account,
        bidder_project_token_account,
        vested_token_escrow,
        token_program,
        token_mint,
        program_authority,
        ..
    } = ctx.accounts;

    // Validate if vested token escrow has enough tokens for transfer
    require!(
        vested_token_escrow.amount >= bidder_vested_account.round_status[index as usize].total_tokens,
        ErrorCode::NotEnoughTokensOnVestedEscrow
    );

    // Validate it this vested account is already claimed
    require!(
        !bidder_vested_account.round_status[index as usize].is_claimed,
        ErrorCode::VestedAlreadyClaimed
    );

    // Construct the program authority signer
    let seeds = &[b"authority", &[program_authority.bump][..]];
    let signer_seeds = &[&seeds[..]];

    transfer_checked(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            TransferChecked {
                from: vested_token_escrow.to_account_info(),
                to: bidder_project_token_account.to_account_info(),
                authority: program_authority.to_account_info(),
                mint: token_mint.to_account_info(),
            },
            signer_seeds,
        ),
        bidder_vested_account.round_status[index as usize].total_tokens,
        token_mint.decimals,
    )?;

    // Update the vested state accounts
    bidder_vested_account.claimed_updated(index);

    Ok(())
}


// TODO!
// Additional constraints(?)
// Should add implementation of vesting schedule on the vesting accounts
// so we can verify vesting schedules
