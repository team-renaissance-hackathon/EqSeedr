#[derive(Accounts)]
pub struct TransferRentZeroCopy<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub new_account: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<TransferRentZeroCopy>) -> Result<()> {
    let TransferRentZeroCopy {
        payer,
        new_account,
        system_program,
    } = ctx.accounts;

    let space = TickBidLeaderBoard::Len;

    let rent = Rent::get()?.minimum_balance(space.try_into().expect("overflow"));

    let (_, new_account_bump_seed) =
        Pubkey::find_program_address(&[b"leader-board", payer.key.as_ref()], &ctx.program_id);

    let seeds = &[
        b"leader-board",
        payer.to_account_info().key.as_ref(),
        &[new_account_bump_seed],
    ];

    let signer_seeds = &[&seeds[..]];

    transfer(
        CpiContext::new_with_signer(
            system_program.to_account_info(),
            Transfer {
                from: payer.to_account_info(),
                to: new_account.to_account_info(),
            },
            signer_seeds,
        ),
        rent,
    )?;
    Ok(())
}
