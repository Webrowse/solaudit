use solana_client::nonblocking::rpc_client::RpcClient;
// use solana_sdk::pubkey::Pubkey;
use anyhow::{Result, anyhow};

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
}
