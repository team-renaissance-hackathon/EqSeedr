#[derive(Accounts)]
pub struct AssignZeroCopy<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub new_account: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AssignZeroCopy>) -> Result<()> {
    let AssignZeroCopy {
        payer,
        new_account,
        system_program,
    } = ctx.accounts;

    let space = amount;

    let (_, new_account_bump_seed) =
        Pubkey::find_program_address(&[b"leader-board", payer.key.as_ref()], &ctx.program_id);

    let seeds = &[
        b"leader-board",
        payer.to_account_info().key.as_ref(),
        &[new_account_bump_seed],
    ];

    let signer_seeds = &[&seeds[..]];

    allocate(
        CpiContext::new_with_signer(
            system_program.to_account_info(),
            Allocate {
                account_to_allocate: new_account.to_account_info(),
            },
            signer_seeds,
        ),
        space,
    )?;

    assign(
        CpiContext::new_with_signer(
            system_program.to_account_info(),
            Assign {
                account_to_assign: new_account.to_account_info(),
            },
            signer_seeds,
        ),
        &ctx.program_id,
    )?;

    Ok(())
}
