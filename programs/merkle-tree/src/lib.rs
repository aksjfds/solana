#![allow(unexpected_cfgs, deprecated)]

use anchor_lang::prelude::*;
use std::hash::{DefaultHasher, Hash, Hasher};

declare_id!("2xR9kA5Xc8eN5S6mHmquQgyHUPrEhdHKWvsHSz4KURmg");

#[program]
pub mod merkle_tree {
    use std::ops::Deref;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, root: u64) -> Result<()> {
        ctx.accounts.root.val = root;

        Ok(())
    }

    pub fn verify(ctx: Context<Verify>, proof: Vec<u64>) -> Result<()> {
        let leaf = ctx.accounts.payer.key();
        let root = ctx.accounts.root.deref();

        let result = root.verify(leaf, proof);
        msg!("result: {:?}", result);

        Ok(())
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[account]
#[derive(InitSpace)]
pub struct Root {
    pub val: u64,
}

impl Root {
    pub fn verify(&self, leaf: Pubkey, proof: Vec<u64>) -> bool {
        let leaf = calculate_hash(&leaf);
        let result = proof
            .into_iter()
            .fold(leaf, |a, b| calculate_hash(&(a + b)));

        if result == self.val {
            true
        } else {
            false
        }
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, payer=payer, space=Root::INIT_SPACE + 16,
        seeds=[b"root"], bump
    )]
    pub root: Account<'info, Root>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Verify<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(seeds=[b"root"], bump)]
    pub root: Account<'info, Root>,
}
