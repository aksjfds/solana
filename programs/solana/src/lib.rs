#![allow(unexpected_cfgs, deprecated)]

use anchor_lang::prelude::*;

declare_id!("Crgf2RxE1xdvcz9iqK3QFWrZnguPmtFuNU9wpJRLvi4d");

#[program]
pub mod solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        msg!("Greetings from: {:?}", amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
