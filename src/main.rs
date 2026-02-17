use anyhow::Result;
use clap::Parser;

mod analysis;
mod cli;
mod models;
mod report;
mod rpc;

use crate::cli::args::Cli;
// use crate::models::types;
use crate::analysis::engine::SnapshotDiff;
use crate::rpc::client::SolanaRpc;
use analysis::engine::{Classify, Classification, RetrySafety};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("Connecting to {}...", cli.cluster);

    let rpc = SolanaRpc::new(&cli.cluster)?;

    println!("Fetching program accounts...");

    let count = rpc.get_program_accounts(&cli.program).await?;

    println!("Found {} accounts", count);

    let health = match rpc.health_check().await {
        Ok(_) => {
            format!("RPC is connected, Tokio Sync is working, Solana v3 client config properly")
        }
        Err(e) => format!("Error :{:?}", e),
    };

    println!("{}", health);

    let lamports = rpc.get_account_lamports(&cli.program).await?;

    println!("Lamports: {:?}", lamports);

    let snap_shot = rpc.fetch_snapshot(&cli.program).await?;

    println!("Snapshot: \n{:#?}", snap_shot);

    let before = rpc.fetch_snapshot(&cli.program).await?;
    // Simulation "After" manually
    let mut after = before.clone();
    after.lamports += 1;

    let diff = SnapshotDiff::diff(&before, &after);

    // println!("Diff result \n{:#?}", diff);
    let result = Classify(&diff);

    println!("Classification");
    println!("{:#?}", result);

    Ok(())
}
