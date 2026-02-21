use crate::models::types::AccountSnapshot;
use anyhow::{Result, anyhow};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_request::RpcRequest;
use solana_sdk::pubkey::Pubkey;

/// Result of a transaction simulation, including simulated account states.
pub struct SimulationResult {
    pub error: Option<String>,
    pub logs: Vec<String>,
    pub post_snapshot: Option<AccountSnapshot>,
    pub units_consumed: Option<u64>,
}

pub struct SolanaRpc {
    client: RpcClient,
}

impl SolanaRpc {
    pub fn new(cluster: &str) -> Result<Self> {
        let url = match cluster {
            "devnet" => "https://api.devnet.solana.com".to_string(),
            "mainnet" => "https://api.mainnet-beta.solana.com".to_string(),
            _ => return Err(anyhow!("Invalid cluster: {}", cluster)),
        };

        let client = RpcClient::new(url);

        Ok(Self { client })
    }

    pub async fn fetch_snapshot(&self, address: &str) -> Result<AccountSnapshot> {
        let pubkey: Pubkey = address.parse()?;

        let account = self.client.get_account(&pubkey).await?;

        Ok(AccountSnapshot {
            pubkey,
            lamports: account.lamports,
            owner: account.owner,
            executable: account.executable,
            data_len: account.data.len(),
            rent_epoch: account.rent_epoch,
        })
    }

    /// Simulate a base64-encoded transaction and return the simulated post-state
    /// for the watched account.
    ///
    /// Uses the RPC `simulateTransaction` with `accounts` config so we get the
    /// simulated account state back in the response — no on-chain state is mutated.
    pub async fn simulate_transaction(
        &self,
        tx_base64: &str,
        watch_address: &str,
    ) -> Result<SimulationResult> {
        // Validate the base64 input decodes to bytes
        STANDARD.decode(tx_base64)
            .map_err(|e| anyhow!("Invalid base64 transaction: {}", e))?;

        // Build the RPC request with accounts config to get post-state back.
        // We pass the base64 string directly — the RPC server deserializes it.
        let params = serde_json::json!([
            tx_base64,
            {
                "encoding": "base64",
                "sigVerify": false,
                "replaceRecentBlockhash": true,
                "accounts": {
                    "encoding": "base64",
                    "addresses": [watch_address]
                }
            }
        ]);

        let response: serde_json::Value = self.client.send(
            RpcRequest::SimulateTransaction,
            params,
        ).await?;

        // Parse error field
        let error = response.get("err")
            .and_then(|v| if v.is_null() { None } else { Some(format!("{}", v)) });

        // Parse logs
        let logs = response.get("logs")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        // Parse compute units consumed
        let units_consumed = response.get("unitsConsumed")
            .and_then(|v| v.as_u64());

        // Parse the simulated account state for our watched address
        let post_snapshot = self.parse_simulated_account(&response, watch_address)?;

        Ok(SimulationResult {
            error,
            logs,
            post_snapshot,
            units_consumed,
        })
    }

    /// Extract the simulated account from the RPC response and convert it
    /// into an AccountSnapshot.
    fn parse_simulated_account(
        &self,
        response: &serde_json::Value,
        address: &str,
    ) -> Result<Option<AccountSnapshot>> {
        let accounts = match response.get("accounts").and_then(|v| v.as_array()) {
            Some(arr) => arr,
            None => return Ok(None),
        };

        // We requested one address, so we expect one entry
        let account_value = match accounts.first().and_then(|v| if v.is_null() { None } else { Some(v) }) {
            Some(v) => v,
            None => return Ok(None),
        };

        let pubkey: Pubkey = address.parse()?;

        let lamports = account_value.get("lamports")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow!("Missing lamports in simulated account"))?;

        let owner: Pubkey = account_value.get("owner")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing owner in simulated account"))?
            .parse()?;

        let executable = account_value.get("executable")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let rent_epoch = account_value.get("rentEpoch")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        // Decode account data to get its length
        let data_len = match account_value.get("data").and_then(|v| v.as_array()) {
            Some(arr) => {
                // Format: ["<base64 data>", "base64"]
                let encoded = arr.first()
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                if encoded.is_empty() {
                    0
                } else {
                    STANDARD.decode(encoded)
                        .map(|bytes| bytes.len())
                        .unwrap_or(0)
                }
            }
            None => 0,
        };

        Ok(Some(AccountSnapshot {
            pubkey,
            lamports,
            owner,
            executable,
            data_len,
            rent_epoch,
        }))
    }
}
