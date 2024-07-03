#[derive(Accounts)]
pub struct InitializeZeroCopy<'info> {
    #[account(zero)]
    pub new_account: AccountLoader<'info, DataAccount>,
}

pub fn handler(ctx: Context<InitializeZeroCopy>, input: u64) -> Result<()> {
    let new_account = &mut ctx.accounts.new_account.load_init()?;
    // new_account.data[0] = input;

    msg!("Initialize data to: {}", input);
    Ok(())
}
