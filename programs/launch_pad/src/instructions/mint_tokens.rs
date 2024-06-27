use crate::states::ProgramAuthority;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{mint_to, Mint, MintTo, TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    // constraint = multi_sig.validate() @ ErrorCode::MissingSigners
    // multi_sig: Account<'info, Approvers>,
    pub program_authority: Box<Account<'info, ProgramAuthority>>,

    #[account(
        mut,
        seeds = [
            program_authority.key().clone().as_ref(),
            b"eqseedr-token-mint",
        ],
        bump
    )]
    pub token_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(mut)]
    pub receipent: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handler(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
    let MintTokens {
        // multi_sig,
        program_authority,
        token_mint,
        receipent,
        token_program,
        ..
    } = ctx.accounts;

    // // multi_sig to approve mint of tokens
    // // amount needs to be approved and fetched through a smart contract
    // // also maybe need to approve the destination
    // needs to be an approved amount from a contract
    // needs to be an approved dest from a contract

    let seeds = &[b"authority", &[program_authority.bump][..]];
    let signer_seeds = &[&seeds[..]];

    mint_to(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            MintTo {
                mint: token_mint.to_account_info(),
                to: receipent.to_account_info(),
                authority: program_authority.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )?;

    Ok(())
}
