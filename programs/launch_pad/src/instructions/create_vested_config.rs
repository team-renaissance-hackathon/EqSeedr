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
        constraint = !session.has_vested_config @ ProgramError::VestedConfigBySessionAlreadyExist
    )]
    pub session: Account<'info, Session>,

    #[account(
        constraint = token_mint.key() == session.data.token_mint @ ProgramError::InvalidTokenMint
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

    new_vested_config.init(ctx.bumps.new_vested_config, session, token_mint);
    session.vested_config_is_set();

    Ok(())
}
