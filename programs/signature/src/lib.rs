#![allow(unexpected_cfgs, deprecated)]

use anchor_lang::solana_program::sysvar::instructions::{
    load_current_index_checked, load_instruction_at_checked,
};
use anchor_lang::{prelude::*, solana_program};

declare_id!("9coYgijAmuE9woESRL6PFXgQhUtJx2Rk3LCVAU2qDCby");

const SIGNATURE_PUBKEY: Pubkey = pubkey!("79kSeCmoWD7s2pwA5xRmHi7SiNmNuZS4i8Z9ZUAHiKD");

#[program]
pub mod signature {

    use super::*;

    pub fn verify(ctx: Context<Verify>) -> Result<()> {
        let instruction_sysvar = &ctx.accounts.sysvar;
        let current_index = load_current_index_checked(instruction_sysvar)?;
        let ed25519_instruction =
            load_instruction_at_checked((current_index - 1) as usize, instruction_sysvar)?;

        let instruction_data = ed25519_instruction.data;
        let offsets: Ed25519SignatureOffsets =
            Ed25519SignatureOffsets::try_from_slice(&instruction_data[2..16])?;

        // Verify public key
        let pubkey_start = offsets.public_key_offset as usize;
        let pubkey_end = pubkey_start + 32;
        let pubkey = &instruction_data[pubkey_start..pubkey_end];
        let pubkey = Pubkey::try_from_slice(pubkey)?;
        require!(pubkey == SIGNATURE_PUBKEY, SignatureError::InvalidPublicKey);

        // Verify message
        let msg_start = offsets.message_data_offset as usize;
        let msg_end = msg_start + offsets.message_data_size as usize;
        let message = &instruction_data[msg_start..msg_end];
        let expected_message = ctx.accounts.payer.key.as_ref();
        require!(message == expected_message, SignatureError::InvalidMessage);

        // do something, mint or transfer.
        // ...

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Verify<'info> {
    pub payer: Signer<'info>,

    #[account(address=solana_program::sysvar::instructions::ID)]
    pub sysvar: AccountInfo<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
struct Ed25519SignatureOffsets {
    signature_offset: u16,
    signature_instruction_index: u16,
    public_key_offset: u16,
    public_key_instruction_index: u16,
    message_data_offset: u16,
    message_data_size: u16,
    message_instruction_index: u16,
}

#[error_code]
enum SignatureError {
    InvalidPublicKey,
    InvalidMessage,
}
