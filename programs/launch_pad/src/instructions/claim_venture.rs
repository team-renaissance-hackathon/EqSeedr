use crate::states::SessionStatus::Closed;
use crate::states::{ProgramAuthority, Session};
use crate::utils::errors::ErrorCode;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

#[derive(Accounts)]
pub struct ClaimVenture<'info> {
    #[account(mut)]
    pub project_owner: Signer<'info>,

    #[account(
        mut,
        constraint = project_owner_token_account.owner == project_owner.key()
            @ErrorCode::InvalidVentureOwner
    )]
    pub project_owner_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        constraint = venture_token_escrow.amount >= session.total_tokens
            @ErrorCode::NotEnoughTokensOnVentureEscrow
        // Ensure there are enough tokens in the escrow account
    )]
    pub venture_token_escrow: InterfaceAccount<'info, TokenAccount>,

    #[account(
        constraint = token_mint.key() == venture_token_escrow.mint
        // Ensure that both the escrow and the token mint have the same mint
    )]
    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        constraint = session.launch_status == Closed
            @ ErrorCode::SessionNotClosed,
        // Ensure that the session is Closed before they can claim venture

        constraint = !session.is_claimed
            @ ErrorCode::VentureTokensAlreadyClaimed,
        // Ensure that the venture tokens of the session isn't claimed yet
    )]
    pub session: Account<'info, Session>,

    #[account(
        seeds = [b"authority"],
        bump = program_authority.bump,
    )]
    pub program_authority: Account<'info, ProgramAuthority>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handler(ctx: Context<ClaimVenture>) -> Result<()> {
    let ClaimVenture {
        session,
        project_owner_token_account,
        venture_token_escrow,
        token_program,
        token_mint,
        program_authority,
        ..
    } = ctx.accounts;

    // Construct the program authority signer
    let seeds = &[b"authority", &[program_authority.bump][..]];
    let signer_seeds = &[&seeds[..]];

    transfer_checked(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            TransferChecked {
                from: venture_token_escrow.to_account_info(),
                to: project_owner_token_account.to_account_info(),
                authority: program_authority.to_account_info(),
                mint: token_mint.to_account_info(),
            },
            signer_seeds,
        ),
        session.total_tokens,
        token_mint.decimals,
    )?;

    // Update the vested state accounts
    session.claimed_update();

    Ok(())
}
