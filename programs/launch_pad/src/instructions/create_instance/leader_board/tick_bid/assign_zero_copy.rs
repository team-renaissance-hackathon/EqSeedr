use crate::states::Session;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{allocate, assign, Allocate, Assign};

#[derive(Accounts)]
pub struct AssignZeroCopy<'info> {
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
        mut,
        constraint = !session.has_tick_bid_leader_board,
    )]
    pub session: Account<'info, Session>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AssignZeroCopy>) -> Result<()> {
    let AssignZeroCopy {
        new_leader_board,
        system_program,
        session,
        ..
    } = ctx.accounts;

    let space = 10240;
    session.tick_bid_leader_board_current_allocation = space;

    let session_key = session.key();
    let seeds = &[
        session_key.as_ref(),
        b"tick-bid-leader-board",
        &[ctx.bumps.new_leader_board][..],
    ];
    let signer_seeds = &[&seeds[..]];

    allocate(
        CpiContext::new_with_signer(
            system_program.to_account_info(),
            Allocate {
                account_to_allocate: new_leader_board.to_account_info(),
            },
            signer_seeds,
        ),
        space,
    )?;

    assign(
        CpiContext::new_with_signer(
            system_program.to_account_info(),
            Assign {
                account_to_assign: new_leader_board.to_account_info(),
            },
            signer_seeds,
        ),
        &ctx.program_id,
    )?;

    Ok(())
}
