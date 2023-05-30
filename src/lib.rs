use anchor_lang::prelude::*;
use bytemuck;
use bytemuck::{Zeroable, Pod};
use anchor_lang::solana_program::pubkey;

const SB_ATTEST_PID: Pubkey = pubkey!("2No5FVKPAAYqytpkEoq93tVh33fo4p6DgAnm4S6oZHo7");

#[zero_copy(unsafe)]
#[repr(packed)]
#[derive(Debug)]
pub struct QuoteAccountData {
    pub delegated_secured_signer: Pubkey,
    pub bump: u8,
    pub quote_registry: [u8; 32],
    pub registry_key: [u8; 64],
    pub attestation_queue: Pubkey,
    pub mr_enclave: [u8; 32],
    pub verification_status: u8,
    pub verification_timestamp: i64,
    pub valid_until: i64,
    pub is_on_queue: bool,
    pub last_heartbeat: i64,
    pub owner: Pubkey,
    pub created_at: i64,
    pub _ebuf: [u8; 992],
}
unsafe impl Pod for QuoteAccountData {}
unsafe impl Zeroable for QuoteAccountData {}
#[zero_copy(unsafe)]
#[repr(packed)]
#[derive(Debug)]
pub struct FunctionAccountData {
    pub name: [u8; 64],
    pub metadata: [u8; 256],
    pub authority: Pubkey,
    ///
    pub container_registry: [u8; 64],
    pub container: [u8; 64],
    pub version: [u8; 32],
    ///
    pub attestation_queue: Pubkey,
    pub queue_idx: u32,
    pub last_execution_timestamp: i64,
    pub next_allowed_timestamp: i64,
    pub schedule: [u8; 64],
    pub escrow: Pubkey,
    pub status: FunctionStatus,
    pub created_at: i64,
    pub _ebuf: [u8; 1024],
}
unsafe impl Pod for FunctionAccountData {}
unsafe impl Zeroable for FunctionAccountData {}
#[repr(u8)]
#[derive(Copy, Debug, Clone, Eq, PartialEq, AnchorSerialize, AnchorDeserialize)]
pub enum FunctionStatus {
    None = 0,
    Active = 1 << 0,
    NonExecutable = 1 << 1,
    Expired = 1 << 2,
    OutOfFunds = 1 << 3,
    InvalidPermissions = 1 << 4,
}

pub fn validate_fn_quote<'a>(func: &AccountInfo<'a>, quote: &AccountInfo<'a>, signer: &AccountInfo<'a>) -> bool {
    if *func.owner != SB_ATTEST_PID {
        return false;
    }
    if *quote.owner != SB_ATTEST_PID {
        return false;
    }
    let quote_buf = quote.try_borrow_data().unwrap();
    let quote_data = bytemuck::from_bytes::<QuoteAccountData>(&quote_buf[8..]);
    let actual_quote_key = Pubkey::create_program_address(
        &[b"QuoteAccountData", func.key().as_ref(), &[quote_data.bump]], &SB_ATTEST_PID).unwrap();
    if quote.key() != actual_quote_key {
        return false;
    }
    if quote_data.delegated_secured_signer != signer.key() {
        return false;
    }
    true
}
