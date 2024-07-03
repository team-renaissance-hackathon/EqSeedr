use crate::states::{LeaderBoard, Prod, Session};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateLeaderBaord<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            session.key().as_ref(),
            b"tick-bid-leader-board",
        ],
        bump,
    )]
    pub leader_board: AccountLoader<'info, LeaderBoard>,

    pub session: Account<'info, Session>,
}

pub fn handler(ctx: Context<UpdateLeaderBaord>) -> Result<()> {
    let leader_board = &mut ctx.accounts.leader_board.load_mut()?;

    let node = Prod {
        amount: 4534,
        bid: 2344,
    };

    let data = node.try_to_vec()?;
    // let data = node;

    leader_board.data[0..16].copy_from_slice(&data);

    let data = &leader_board.data[4..20];

    let mut node = Prod::try_from_slice(data)?;

    // node.amount = 534556345;
    node.bid = 1;
    let data = node.try_to_vec()?;
    leader_board.data[0..16].copy_from_slice(&data);

    // leader_board.data[8..12].copy_from_slice(&[120, 130, 140, 250]);

    // leader_board.data[21..23].copy_from_slice(&[12, 13]);

    Ok(())
}
