#[derive(Accounts)]
pub struct ReallocZeroCopy<'info> {
    #[account(mut)]
    /// CHECKED: testing
    pub new_account: AccountInfo<'info>,
}

pub fn handler(ctx: Context<ReallocateZeroCopy>) -> Result<()> {
    let ReallocateZeroCopy { new_account } = ctx.accounts;

    let space = amount;

    new_account.realloc(space as usize, false)?;

    Ok(())
}
