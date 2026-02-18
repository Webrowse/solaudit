use anyhow::Result;
use clap::Parser;


mod analysis;
mod cli;
mod models;
mod report;
mod rpc;


use crate::analysis::engine::analyse;
use crate::cli::args::Cli;
use crate::rpc::client::SolanaRpc;
use crate::report::writer::print_text;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let rpc = SolanaRpc::new(&cli.cluster)?;

    // Pre-state
    let before = rpc.fetch_snapshot(&cli.program).await?;

    // Temp: fake change in state
    let mut after = before.clone();
    after.lamports += 1;

    // Analyse
    let result = analyse(before, after);

    // Report
    print_text(&result);
    
    Ok(())
}
