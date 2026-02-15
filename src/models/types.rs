use solana_sdk::pubkey::Pubkey;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSnapshot {
    pub pubkey: Pubkey,
    pub lamport: u64,
    pub owner: Pubkey,
    pub executable: bool,
    pub data_len: usize,
    pub rent_epoch: u64,
}