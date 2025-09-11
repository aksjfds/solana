use anchor_lang::prelude::*;
use anchor_spl::token::{
    set_authority, spl_token::instruction::AuthorityType, Mint, SetAuthority, Token,
};

pub fn start(
    ctx: Context<Start>,
    start_price: u64,
    end_price: u64,
    duration: u64,
    drop_interval: u64,
    drop_step: u64,
) -> Result<()> {
    let clock = Clock::get()?;
    let start_time = clock.unix_timestamp as u64;
    let end_time = start_time + duration;

    *ctx.accounts.auction_info = AuctionInfo {
        start_price,
        end_price,
        start_time,
        end_time,
        drop_interval,
        drop_step,
    };

    msg!("auction_info: {:?}", ctx.accounts.auction_info);

    // change mint's authority
    let seeds: &[&[u8]] = &[b"mint"];
    let (new_authority, _) = Pubkey::find_program_address(seeds, &crate::ID);
    let cpi_accounts = SetAuthority {
        current_authority: ctx.accounts.payer.to_account_info(),
        account_or_mint: ctx.accounts.mint.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context: _ = CpiContext::new(cpi_program, cpi_accounts);

    set_authority(cpi_context, AuthorityType::MintTokens, Some(new_authority))?;

    Ok(())
}

#[account]
#[derive(InitSpace, Debug)]
pub struct AuctionInfo {
    pub start_price: u64,
    pub end_price: u64,

    pub start_time: u64,
    pub end_time: u64,

    pub drop_interval: u64,
    pub drop_step: u64,
}

#[derive(Accounts)]
pub struct Start<'info> {
    #[account(
        init,
        payer=payer,
        space=8 + AuctionInfo::INIT_SPACE,
        seeds=[b"auction"],
        bump
    )]
    pub auction_info: Account<'info, AuctionInfo>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}