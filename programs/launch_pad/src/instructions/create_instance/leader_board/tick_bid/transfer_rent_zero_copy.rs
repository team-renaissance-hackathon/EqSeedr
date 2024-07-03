use crate::states::{LeaderBoard, Session};
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

#[derive(Accounts)]
pub struct TransferRentZeroCopy<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            session.key().as_ref(),
            b"tick-bid-leader-board",
        ],
        bump,
    )]
    pub new_leader_board: SystemAccount<'info>,

    #[account(
        constraint = !session.has_tick_bid_leader_board,
    )]
    pub session: Account<'info, Session>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<TransferRentZeroCopy>) -> Result<()> {
    let TransferRentZeroCopy {
        payer,
        new_leader_board,
        system_program,
        ..
    } = ctx.accounts;

    let space = LeaderBoard::LEN;

    let rent = Rent::get()?.minimum_balance(space.try_into().expect("overflow"));

    transfer(
        CpiContext::new(
            system_program.to_account_info(),
            Transfer {
                from: payer.to_account_info(),
                to: new_leader_board.to_account_info(),
            },
        ),
        rent,
    )?;
    Ok(())
}
