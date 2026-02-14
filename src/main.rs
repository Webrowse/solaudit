use clap::Parser;
use anyhow::Result;

mod cli;
mod rpc;
mod analysis;
mod report;
mod models;

use crate::cli::args::Cli;
use crate::rpc::client::SolanaRpc;

#[tokio::main]
async fn main() -> Result<()> {

    let cli = Cli::parse();
    
    println!("Connecting to {}...", cli.cluster);
    
    let rpc = SolanaRpc::new(&cli.cluster)?;

    println!("Fetching program accounts...");
    
    let count = rpc.get_program_accounts(&cli.program).await?;
    
    println!("Found {} accounts", count);
    


    Ok(())
}
