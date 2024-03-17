#[derive(Accounts)]
pub struct CreateVestingEscrowAccount<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = VestingEscrowAccount::LEN,
        seeds = [
            session.key().as_ref(),
            b"vesting-escrow-account",
        ],
        bump
    )]
    pub new_vesting_escrow_account: Account<'info, VestingEscrowAccount>,

    #[account(
        mut,
        has_one = authority
        constraint = !session.has_vesting_escrow_account @ ProgramError::VestCreateVestingEscrowAccountAlreadyExist
        constraint = session.data.token_mint == token_mint.key() @ ProgramError::InvalidTokenMint
    )]
    pub session: Account<'info, Session>,

    pub token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateVestingEscrowAccount>) -> Result<()> {
    let CreateVestingEscrowAccount {
        new_vesting_escrow_account,
        session,
        ..
    } = ctx.accounts;

    // need set state logic

    Ok(())
}
