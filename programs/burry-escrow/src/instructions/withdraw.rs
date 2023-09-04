use crate::state::*;
use crate::errors::*;
use std::str::FromStr;
use anchor_lang::prelude::*;
use switchboard_v2::{AggregatorAccountData, SwitchboardDecimal};
use anchor_lang::solana_program::clock::Clock;

pub fn withdraw_handler(ctx: Context<Withdraw>, params: WithdrawParams) -> Result<()> {
    let feed = &ctx.accounts.feed_aggregator.load()?;
    let escrow_state = &ctx.accounts.escrow_account;

    // get result
    let val: f64 = feed.get_result()?.try_into()?;
    let current_timestamp = Clock::get().unwrap().unix_timestamp;
    let mut valid_transfer: bool = false;


    msg!("Current feed result is {}!", val);
    msg!("Unlock price is {}", escrow_state.unlock_price);

    if (current_timestamp - feed.latest_confirmed_round.round_open_timestamp) > 86400 {
        valid_transfer = true;
    } else if **ctx.accounts.feed_aggregator.to_account_info().try_borrow_lamports()? == 0 {
        valid_transfer = true;
    } else if val > escrow_state.unlock_price as f64 {
    // Normal Use Case

        // check feed does not exceed max_confidence_interval
        if let Some(max_confidence_interval) = params.max_confidence_interval {
            feed.check_confidence_interval(SwitchboardDecimal::from_f64(max_confidence_interval))
                .map_err(|_| error!(EscrowErrorCode::ConfidenceIntervalExceeded))?;
        }

        feed.check_staleness(current_timestamp, 300)
        .map_err(|_| error!(EscrowErrorCode::StaleFeed))?;

        valid_transfer = true;
    }
    
    if valid_transfer{
        **escrow_state.to_account_info().try_borrow_mut_lamports()? = escrow_state
            .to_account_info()
            .lamports()
            .checked_sub(escrow_state.escrow_amount)
            .ok_or(ProgramError::InvalidArgument)?;

        **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? = ctx.accounts.user
            .to_account_info()
            .lamports()
            .checked_add(escrow_state.escrow_amount)
            .ok_or(ProgramError::InvalidArgument)?;
    } else {
        return Err(error!(EscrowErrorCode::InvalidWithdrawalRequest));
    }

    Ok(())
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
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
    // Switchboard SOL feed aggregator
    #[account(
        address = Pubkey::from_str(SOL_USDC_FEED).unwrap()
    )]
    pub feed_aggregator: AccountLoader<'info, AggregatorAccountData>,
    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct WithdrawParams {
    pub max_confidence_interval: Option<f64>,
}