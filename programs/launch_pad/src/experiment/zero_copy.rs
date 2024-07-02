use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateZeroCopy<'info> {
    #[account(mut)]
    pub existing_account: AccountLoader<'info, DataAccount>,
}

#[account(zero_copy)]
// #[derive(Default)]
pub struct DataAccount {
    // pub data: [u8; 10485760],
    pub data: [u8; 3064],
    // pub data: *mut u8,
}

pub fn update_zero_copy(ctx: Context<UpdateZeroCopy>, input: u64) -> Result<()> {
    let existing_account = &mut ctx.accounts.existing_account.load_mut()?;
    // existing_account.data[1] = input;

    msg!("Updated data to: {}", input);
    Ok(())
}
