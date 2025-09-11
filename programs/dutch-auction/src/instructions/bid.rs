use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::token::spl_token::instruction::AuthorityType;
use anchor_spl::token::{mint_to, set_authority, Mint, MintTo, SetAuthority, Token, TokenAccount};

use crate::{error::AuctionError, AuctionInfo};

pub fn bid(ctx: Context<Bid>, amount: u64) -> Result<()> {
    // get current price
    let info = &ctx.accounts.auction_info;
    let current_time = Clock::get()?.unix_timestamp as u64;
    let steps = (current_time - info.start_time) / info.drop_interval;
    let current_price = info.start_price - (steps * info.drop_step);
    require!(amount > current_price, AuctionError::InsufficientSolError);

    // transfer
    let cpi_accounts = Transfer {
        from: ctx.accounts.payer.to_account_info(),
        to: ctx.accounts.seller.to_account_info(),
    };
    let cpi_program = ctx.accounts.system_program.to_account_info();
    let cpi_context: _ = CpiContext::new(cpi_program, cpi_accounts);
    transfer(cpi_context, amount)?;

    // mint to
    let signer_seeds: &[&[&[u8]]] = &[&[b"mint", &[ctx.bumps.mint_authority]]];
    mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.pda.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
        )
        .with_signer(signer_seeds),
        1,
    )?;

    // close mint
    let cpi_accounts = SetAuthority {
        current_authority: ctx.accounts.mint_authority.to_account_info(),
        account_or_mint: ctx.accounts.mint.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context: _ = CpiContext::new(cpi_program, cpi_accounts).with_signer(signer_seeds);

    set_authority(cpi_context, AuthorityType::MintTokens, None)?;

    Ok(())
}

#[derive(Accounts)]
pub struct Bid<'info> {
    #[account(
        init,
        payer = payer,
        token::mint = mint,
        token::authority = pda,
        seeds=[b"pda"],
        bump
    )]
    pub pda: Account<'info, TokenAccount>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(seeds=[b"mint"], bump)]
    pub mint_authority: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub seller: SystemAccount<'info>,

    #[account(seeds=[b"auction"], bump)]
    pub auction_info: Account<'info, AuctionInfo>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}