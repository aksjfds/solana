#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;

#[allow(ambiguous_glob_reexports)]
pub use instructions::*;

declare_id!("25MgjiKAVySForPXcZdizT4jPa1jcSPGhdfJUfLStGsz");

#[program]
pub mod dutch_auction {
    use super::*;

    pub fn start(
        ctx: Context<Start>,
        start_price: u64,
        end_price: u64,
        duration: u64,
        drop_interval: u64,
        drop_step: u64,
    ) -> Result<()> {
        instructions::start(
            ctx,
            start_price,
            end_price,
            duration,
            drop_interval,
            drop_step,
        )
    }

    pub fn bid(ctx: Context<Bid>, amount: u64) -> Result<()> {
        instructions::bid(ctx, amount)
    }

}