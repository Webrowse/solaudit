use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSnapshot {
    pub pubkey: Pubkey,
    pub lamports: u64,
    pub owner: Pubkey,
    pub executable: bool,
    pub data_len: usize,
    pub rent_epoch: u64,
}
