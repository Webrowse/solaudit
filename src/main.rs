use clap::Parser;
use anyhow::Result;

mod cli;
mod rpc;
mod analysis;
mod report;
mod models;

use crate::cli::args::Cli;

fn main() -> Result<()> {

    let cli = Cli::parse();

    println!("Running solaudit");
    println!("Program: {}", cli.program);
    println!("Cluster: {}", cli.cluster);
    println!("Output: {}", cli.output);
    
    Ok(())
}
