use crate::states::ProgramAuthority;

use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

#[derive(Accounts)]
pub struct AddBidTokenMint<'info> {
    // shuld change this in the future so that the authority comes from
    // an approved multisig account
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority,
    )]
    pub program_authority: Account<'info, ProgramAuthority>,

    pub token_mint: InterfaceAccount<'info, Mint>,
}

pub fn handler(ctx: Context<AddBidTokenMint>) -> Result<()> {
    let AddBidTokenMint {
        program_authority,
        token_mint,
        ..
    } = ctx.accounts;

    program_authority.add_bid_token_mint(token_mint.key());

    // log event that a new bid token mint has been added.
    Ok(())
}

// TODO!
// - need to implement event logs.
// - need to change to using multisig for authority
