#[derive(Accounts)]
pub struct CreateVestedEscrowAccountBySession<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = VestedEscrowAccountBySession::LEN,
        seeds = [
            session.key().as_ref(),
            b"Vested-escrow-account",
        ],
        bump
    )]
    pub new_Vested_escrow_account: Account<'info, VestedEscrowAccountBySession>,

    #[account(
        mut,
        has_one = authority
        constraint = !session.has_Vested_escrow_account @ ProgramError::VestedEscrowAccountBySessionAlreadyExist
        constraint = session.data.token_mint == token_mint.key() @ ProgramError::InvalidTokenMint
    )]
    pub session: Account<'info, Session>,

    pub token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateVestedEscrowAccountBySession>) -> Result<()> {
    let CreateVestedEscrowAccountBySession { session, .. } = ctx.accounts;

    session.vested_escrow_account_is_set();

    Ok(())
}
