use anchor_lang::prelude::*;
use instructions::deposit::*;
use instructions::withdraw::*;
use instructions::withdraw_closed_feed::*;

pub mod instructions;
pub mod state;
pub mod errors;

declare_id!("43ELGXdjDpGoKuiepke5ghpDy6Mudr7qfJ7oV57cw82b");

#[program]
mod burry_escrow {

    use super::*;

    pub fn deposit(ctx: Context<Deposit>, escrow_amt: u64, unlock_price: f64) -> Result<()> {
        deposit_handler(ctx, escrow_amt, unlock_price)
    }

    pub fn withdraw(ctx: Context<Withdraw>, params: WithdrawParams) -> Result<()> {
        withdraw_handler(ctx, params)
    }

    pub fn withdraw_closed_feed(ctx: Context<WithdrawClosedFeed>) -> Result<()> {
        withdraw_closed_feed_handler(ctx)
    }
}
