#![allow(unexpected_cfgs, deprecated)]

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

declare_id!("Gqej27KfZogJP1azTKwcTco3zh9y254Fu6vEVwMTV3F7");

#[program]
pub mod nft_exchange {

    use anchor_spl::token::{set_authority, spl_token::instruction::AuthorityType, SetAuthority};

    use super::*;

    pub fn sell(ctx: Context<Sell>, price: u64) -> Result<()> {
        // change nft's authority

        set_authority(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                SetAuthority {
                    current_authority: ctx.accounts.seller.to_account_info(),
                    account_or_mint: ctx.accounts.nft_account.to_account_info(),
                },
            ),
            AuthorityType::AccountOwner,
            Some(ctx.accounts.order.key()),
        )?;

        // create order
        *ctx.accounts.order = Order {
            mint: ctx.accounts.mint.key(),
            nft_account: ctx.accounts.nft_account.key(),
            price: price,
        };

        Ok(())
    }

    pub fn buy(ctx: Context<Buy>) -> Result<()> {
        let mint = ctx.accounts.mint.key();
        let nft = ctx.accounts.nft_account.key();
        let signer_seeds: &[&[&[u8]]] = &[&[mint.as_ref(), nft.as_ref(), &[ctx.bumps.order]]];

        // transfer
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Order {
    pub mint: Pubkey,
    pub nft_account: Pubkey,
    pub price: u64,
}

#[derive(Accounts)]
pub struct Sell<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(mint::decimals = 0)]
    pub mint: Account<'info, Mint>,

    #[account(mut, token::mint=mint, token::authority=seller)]
    pub nft_account: Account<'info, TokenAccount>,

    #[account(
        init,
        payer=seller,
        space=Order::INIT_SPACE + 8,
        seeds=[mint.key().as_ref(), nft_account.key().as_ref()],
        bump
    )]
    pub order: Account<'info, Order>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Buy<'info> {
    pub buyer: Signer<'info>,

    pub mint: Account<'info, Mint>,

    #[account(mut, token::mint=mint)]
    pub nft_account: Account<'info, TokenAccount>,

    #[account(
        seeds=[mint.key().as_ref(), nft_account.key().as_ref()],
        bump
    )]
    pub order: Account<'info, Order>,
}
