use anchor_lang::prelude::*;
use instructions::deposit::*;
use instructions::withdraw::*;

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

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        withdraw_handler(ctx)
    }
}
