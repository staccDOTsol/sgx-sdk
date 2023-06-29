use anchor_lang::prelude::*;
use switchboard_solana::AttestationQueueAccountData;

pub mod actions;
pub use actions::*;

declare_id!("GSbisrD6tyxMkUJgCXWMmDz6S8nYzcabdinPujcfu5zF");

const PROGRAM_SEED: &[u8] = b"BINANCEORACLE";

#[program]
pub mod binance_oracle {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Initialize::actuate(&ctx)
    }

    // #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn push_data(ctx: Context<PushData>, params: PushDataParams) -> Result<()> {
        PushData::actuate(&ctx, &params)
    }
}

#[account(zero_copy)]
#[repr(packed)]
#[derive(Default, Debug)]
pub struct ProgramState {
    pub bump: u8,
    pub authority: Pubkey,
    pub attestation_queue: Pubkey,
    pub mr_enclaves: [[u8; 32]; 32],
}

#[account(zero_copy)]
#[repr(packed)]
#[derive(Default, Debug)]
pub struct OracleState {
    pub bump: u8,
    pub symbol: [u8; 32],
    pub price: f64,
    pub volume: f64,
    pub oracle_timestamp: i64,
    pub timestamp: i64,
}

pub struct Row {
    pub price: f64,
    pub volume: f64,
    pub timestamp: i64,
}

#[error_code]
#[derive(Eq, PartialEq)]
pub enum BinanceOracleError {
    #[msg("Stale data")]
    StaleData,
    #[msg("Invalid trusted signer")]
    InvalidTrustedSigner,
    #[msg("Invalid MRENCLAVE")]
    InvalidMrEnclave,
}
