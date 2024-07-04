use crate::states::{round_leader_board::Position, LeaderBoard, Session, VestedAccountByOwner};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(src: u32, dest: u32)]
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

    #[account(
        constraint = vested_account_by_owner.session == session.key(),

        constraint = LeaderBoard::is_valid_src(
            &leader_board, 
            src, 
            &vested_account_by_owner).unwrap(),

        constraint = LeaderBoard::is_valid_dest(
            &leader_board, 
            dest, 
            &vested_account_by_owner).unwrap(),
    )]
    pub vested_account_by_owner: Box<Account<'info, VestedAccountByOwner>>,

    #[account(
        // constraint == session.launch_status.is_valid_tick_bid_status(),
    )]
    pub session: Account<'info, Session>,
}

pub fn handler(ctx: Context<UpdateLeaderBaord>, src: u32, dest: u32) -> Result<()> {
    let leader_board = &mut ctx.accounts.leader_board.load_mut()?;
    let round_index = leader_board.round as usize;

    let (vested_index, avg_bid) = ctx
        .accounts
        .vested_account_by_owner
        .get_avg_bid_by_round(round_index);

    let position = Position {
        vested_index,
        avg_bid,
    };

    if src == leader_board.next_index() {
        leader_board.add(dest, position)?;
    } else if src != dest {
        leader_board.swap(src, dest, position)?;
    } else {
        let mut node = leader_board.read(dest as usize);
        node.position = position;
        leader_board.update(&node)?;
    }

    Ok(())
}
