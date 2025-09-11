#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{mint_to, MintTo};
use anchor_spl::token::{Mint, Token, TokenAccount};

declare_id!("4h1ELKdFXCYHAB3pvEQgyxExNqVUWz5ZZVjKP516VBjA");

#[program]
pub mod faucet {

    use anchor_spl::token::{self, Transfer};

    use super::*;

    pub fn create_faucet(ctx: Context<CreateFaucetAccount>, amount: u64) -> Result<()> {
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            MintTo {
                authority: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.faucet.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
            },
        );

        mint_to(cpi_context, amount)?;

        Ok(())
    }

    pub fn request_tokens(ctx: Context<RequestTokens>) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[b"faucet", &[ctx.bumps.faucet]]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.faucet.to_account_info(),
            to: ctx.accounts.pda.to_account_info(),
            authority: ctx.accounts.faucet.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(signer_seeds);

        token::transfer(cpi_context, 100)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct RequestTokens<'info> {
    // associated_token_account
    #[account(
        init,
        payer = payer,
        token::mint = mint,
        token::authority = payer,
        seeds=[b"pda", crate::ID_CONST.as_ref()],
        bump
    )]
    pub pda: Account<'info, TokenAccount>,

    // faucet_account
    #[account(
        mut,
        token::mint=mint,
        token::authority=faucet,
        seeds=[b"faucet"],
        bump
    )]
    pub faucet: Account<'info, TokenAccount>,

    // payer
    #[account(mut)]
    pub payer: Signer<'info>,
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct CreateFaucetAccount<'info> {
    // faucet_account
    #[account(
        init,
        payer=payer,
        token::mint=mint,
        token::authority=faucet,
        seeds=[b"faucet"],
        bump
    )]
    pub faucet: Account<'info, TokenAccount>,

    // mint
    #[account(mut, mint::authority=payer)]
    pub mint: Account<'info, Mint>,

    // payer
    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}