use anchor_lang::prelude::*;
use instructions::deposit::*;
use instructions::withdraw::*;
use instructions::init_vrf_client::*;
use instructions::get_out_of_jail::*;
use instructions::consume_randomness::*;

pub mod instructions;
pub mod state;
pub mod errors;

declare_id!("43ELGXdjDpGoKuiepke5ghpDy6Mudr7qfJ7oV57cw82b");

#[program]
mod burry_escrow {

    use crate::instructions::init_vrf_client::init_vrf_client_handler;

    use super::*;

    pub fn deposit(ctx: Context<Deposit>, escrow_amt: u64, unlock_price: f64) -> Result<()> {
        deposit_handler(ctx, escrow_amt, unlock_price)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        withdraw_handler(ctx)
    }

    pub fn init_vrf_client(ctx: Context<InitVrfClient>) -> Result<()>{
        init_vrf_client_handler(ctx)
    }

    pub fn get_out_of_jail(ctx: Context<RequestRandomness>, params: RequestRandomnessParams) -> Result<()>{
        get_out_of_jail_handler(ctx, params)
    }

    pub fn consume_randomness(ctx: Context<ConsumeRandomness>) -> Result<()>{
        consume_randomness_handler(ctx)
    }
}
