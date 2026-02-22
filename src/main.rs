use anyhow::Result;
use clap::Parser;

mod analysis;
mod cli;
mod models;
mod report;
mod rpc;

use crate::analysis::engine::analyse;
use crate::cli::args::Cli;
use crate::report::writer::{print_text, print_json};
use crate::rpc::client::SolanaRpc;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let rpc = SolanaRpc::new(&cli.cluster)?;

    // Pre-state: fetch current on-chain snapshot
    let before = rpc.fetch_snapshot(&cli.program).await?;

    // Post-state: either from simulation or same as pre-state
    let after = if let Some(tx_base64) = &cli.tx {
        let sim = rpc.simulate_transaction(tx_base64, &cli.program).await?;

        if let Some(err) = &sim.error {
            eprintln!("Simulation error: {}", err);
        }

        if let Some(units) = sim.units_consumed {
            eprintln!("Compute units consumed: {}", units);
        }

        // Use the simulated post-state if available, otherwise fall back to pre-state
        sim.post_snapshot.unwrap_or_else(|| before.clone())
    } else {
        // No transaction provided — compare account to itself
        before.clone()
    };

    let result = analyse(before, after);

    // print_text(&result);
    match cli.output.as_str() {
        "json" => print_json(&result),
        _ => print_text(&result),
    }

    Ok(())
}
