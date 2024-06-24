use crate::states::{
    // STATE ACCOUNTS
    Session,
    VestedAccountByIndex,
    VestedAccountByOwner,
    VestedConfig,
};

use crate::utils::errors::ProgramError;

use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct SessionRegistration<'info> {
    // investor
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = VestedAccountByOwner::LEN,
        seeds = [
            authority.key().as_ref(),
            session.clone().key().as_ref(),
            b"vested-account-by-owner",
        ],
        bump
    )]
    pub new_vested_account_by_owner: Box<Account<'info, VestedAccountByOwner>>,

    #[account(
        init,
        payer = authority,
        space = VestedAccountByIndex::LEN,
        seeds = [
            vested_config.next_index().as_ref(),
            session.clone().key().as_ref(),
            b"vested-account-by-index",
        ],
        bump
    )]
    pub new_vested_account_by_index: Box<Account<'info, VestedAccountByIndex>>,

    #[account(
        mut,
        constraint = vested_config.session == session.key()
            @ ProgramError::InvalidVestedConfig,
    )]
    pub vested_config: Box<Account<'info, VestedConfig>>,

    pub session: Box<Account<'info, Session>>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SessionRegistration>) -> Result<()> {
    let SessionRegistration {
        authority,
        vested_config,
        new_vested_account_by_owner,
        new_vested_account_by_index,
        session,
        ..
    } = ctx.accounts;

    vested_config.update_index();

    new_vested_account_by_index.initialize(
        ctx.bumps.new_vested_account_by_index,
        vested_config.vested_index,
        authority.key(),
        session.key(),
        vested_config.key(),
    );

    new_vested_account_by_owner.initialize(
        ctx.bumps.new_vested_account_by_owner,
        vested_config.vested_index,
        authority.key(),
        session.key(),
        vested_config.key(),
    );

    msg!(
        "Vested Accounts Created: {}: {}, \n{}, \n{}",
        // accounts
        "\nInvestor",
        authority.key(),
        new_vested_account_by_index.key(),
        new_vested_account_by_owner.key(),
    );

    Ok(())
}

// NOTES:
//  - there should be a cost to register, to reduce spam.
