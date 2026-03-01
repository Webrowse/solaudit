use anyhow::{anyhow, Result};
use clap::Parser;

use solaudit::analysis::engine::analyse;
use solaudit::cli::args::Cli;
use solaudit::report::writer::{print_json, print_text};
use solaudit::rpc::client::SolanaRpc;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let rpc = SolanaRpc::new(&cli.cluster)?;

    let before = rpc
        .fetch_snapshot(&cli.program)
        .await
        .map_err(|e| anyhow!("Failed to fetch pre-state: {}", e))?;

    let (after, simulation_logs) = if let Some(tx_base64) = &cli.tx {
        let sim = rpc.simulate_transaction(tx_base64, &cli.program).await?;

        if let Some(err) = &sim.error {
            eprintln!("Simulation error: {}", err);
        }

        if let Some(units) = sim.units_consumed {
            eprintln!("Compute units consumed: {}", units);
        }

        let after = sim.post_snapshot.unwrap_or_else(|| before.clone());
        (after, sim.logs)
    } else {
        (before.clone(), Vec::new())
    };

    let result = analyse(before, after, simulation_logs);

    match cli.output.as_str() {
        "json" => print_json(&result),
        _ => print_text(&result),
    }

    Ok(())
}
