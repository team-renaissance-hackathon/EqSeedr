use crate::states::{Session, VestedConfigBySession};
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[derive(Accounts)]
pub struct CreateVestedConfigBySession<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = VestedConfigBySession::LEN,
        seeds = [
            session.key().as_ref(),
            b"vested-config",
        ],
        bump
    )]
    pub new_vested_config: Account<'info, VestedConfigBySession>,

    #[account(
        mut,
        has_one = authority,
        constraint = !session.has_vested_config 
        // @ ProgramError::VestedConfigBySessionAlreadyExist
    )]
    pub session: Account<'info, Session>,

    #[account(
        constraint = token_mint.key() == session.token_mint 
        // @ ProgramError::InvalidTokenMint
    )]
    pub token_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateVestedConfigBySession>) -> Result<()> {
    let CreateVestedConfigBySession {
        new_vested_config,
        session,
        token_mint,
        ..
    } = ctx.accounts;

    new_vested_config.initialize(ctx.bumps.new_vested_config, &session.clone(), token_mint.key());
    session.add_vested_config_by_session();

    Ok(())
}
