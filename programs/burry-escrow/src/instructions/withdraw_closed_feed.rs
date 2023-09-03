use crate::state::*;
use crate::errors::*;
use anchor_lang::prelude::*;
use std::str::FromStr;

pub fn withdraw_closed_feed_handler(ctx: Context<WithdrawClosedFeed>) -> Result <()> {

    let escrow_state = &ctx.accounts.escrow_account;
    let user = &ctx.accounts.user;

    msg!("Feed account lamports: {}", **ctx.accounts.closed_feed_account.try_borrow_lamports()?);

    **escrow_state.to_account_info().try_borrow_mut_lamports()? = escrow_state
        .to_account_info()
        .lamports()
        .checked_sub(escrow_state.escrow_amount)
        .ok_or(ProgramError::InvalidArgument)?;

    **user.to_account_info().try_borrow_mut_lamports()? = user
        .to_account_info()
        .lamports()
        .checked_add(escrow_state.escrow_amount)
        .ok_or(ProgramError::InvalidArgument)?;

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawClosedFeed<'info> {
     // user account
    #[account(mut)]
    pub user: Signer<'info>,
    // escrow account
    #[account(
        mut,
        seeds = [ESCROW_SEED, user.key().as_ref()],
        bump,
        close = user
    )]
    pub escrow_account: Account<'info, EscrowState>,
    /// CHECK: comment out the address=SOL_USDC_FEED line to test this instruction
    #[account(
        address = Pubkey::from_str(SOL_USDC_FEED).unwrap(),
        constraint = **closed_feed_account.to_account_info().try_borrow_lamports()? == 0
        @ EscrowErrorCode::FeedAccountIsNotClosed
    )]
    pub closed_feed_account: AccountInfo<'info>
}