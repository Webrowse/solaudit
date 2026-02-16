use crate::models::types::AccountSnapshot;
use anyhow::{Result, anyhow};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

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

    pub async fn get_program_accounts(&self, program: &str) -> Result<usize> {
        let program_id = program.parse()?;

        let accounts = self.client.get_program_accounts(&program_id).await?;

        Ok(accounts.len())
    }

    pub async fn health_check(&self) -> Result<()> {
        let version = self.client.get_version().await?;
        println!("Version of RPC: {:?}", version);

        Ok(())
    }

    pub async fn get_account_lamports(&self, address: &str) -> Result<u64> {
        let pubkey: Pubkey = address.parse()?;
        let account = self.client.get_account(&pubkey).await?;

        Ok(account.lamports)
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
}
