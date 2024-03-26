#[derive(Accounts)]
pub struct AddTokenMint<'info> {
    // should be a multisig -> if multisig need payer?
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority,
    )]
    pub program_authority: Account<'info, ProgramAuthority>,

    // most likely the commit queue token account -> one instance
    // should there be multiple instances at the session level?
    // may need multiple instances of this.
    // but for now this is good enough to demo.
    // in future we can change it.
    #[account(
        init,
        payer = authority,
        associated_token::authority = program_authority,
        associated_token::mint = token_mint,
        associated_token::token_program = token_program,
    )]
    pub program_token_account: Account<'info, TokenAccount>,

    pub token_mint: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AddTokenMint>) -> Result<()> {
    let AddTokenMint {
        program_authority,
        token_mint,
        ..
    } = ctx.accounts;

    program_authority.add_token_account(token_mint.key());
    Ok(())
}
