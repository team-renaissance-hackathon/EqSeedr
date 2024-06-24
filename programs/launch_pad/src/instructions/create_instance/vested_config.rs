use crate::states::{Session, VestedConfig};
use crate::utils::errors::ErrorCode;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

#[derive(Accounts)]
pub struct CreateVestedConfig<'info> {
    // session creator
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [
            token_mint.key().as_ref(),
            b"escrow",
        ],
        bump,
    )]
    pub escrow_authority: SystemAccount<'info>,

    #[account(
        associated_token::mint = token_mint,
        associated_token::authority = escrow_authority,
        associated_token::token_program = token_program,
    )]
    pub vested_token_escrow: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = authority,
        space = VestedConfig::LEN,
        seeds = [
            session.key().as_ref(),
            b"vested-config",
        ],
        bump
    )]
    pub new_vested_config: Account<'info, VestedConfig>,

    #[account(
        mut,
        has_one = authority,
        constraint = !session.has_vested_config 
            @ ErrorCode::VestedConfigAlreadyExist,
    )]
    pub session: Account<'info, Session>,

    #[account(
        constraint = token_mint.key() == session.token_mint 
            @ ErrorCode::InvalidTokenMint,
    )]
    pub token_mint: InterfaceAccount<'info, Mint>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateVestedConfig>) -> Result<()> {
    let CreateVestedConfig {
        new_vested_config,
        session,
        token_mint,
        vested_token_escrow,
        ..
    } = ctx.accounts;

    new_vested_config.initialize(
        ctx.bumps.new_vested_config, 
        &session, 
        token_mint.key(),
        vested_token_escrow.key()
    );

    session.add_vested_config_by_session();

    Ok(())
}

// file name: option, which to choose?
//  - vested_config
//  - vested_status

// design decision:
//  - vested token escrow seeds will not be tied to the vested config
//  - so one instance of the vested token escrow exist
//  - the authority seeds doesn't have to be tied to the session
//  - this allows the flexiability to have multiple funding sessions tied to a specific token mint.
