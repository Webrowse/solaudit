use clap::{Parser};
use anyhow::Result;

#[derive(Parser, Debug)]
#[command(
    name = "solaudit",
    version,
    about = "Solana program audit and retry-safety analyzer"
)]
struct Cli {
    /// Program ID to analyse
    #[arg(long)]
    program: String,

    /// Target cluster: devnet | mainnet
    #[arg(long, default_value = "devnet")]
    cluster: String,

    /// Output format: json | text
    #[arg(long, default_value = "text")]
    output: String,
}

fn main() -> Result<()> {

    let cli = Cli::parse();

    println!("Running solaudit");
    println!("Program: {}", cli.program);
    println!("Cluster: {}", cli.cluster);
    println!("Output: {}", cli.output);
    
    Ok(())
}
