use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "solaudit",
    version = "0.1.0",
    about = "Solana audit and retry-safety tool"
)]
pub struct Cli {
    /// Program ID to analyse
    #[arg(long)]
    pub program: String,

    /// Target cluster ( devnet or mainnet)
    #[arg(long, default_value = "devnet")]
    pub cluster: String,

    /// Output format (Json or text)
    #[arg(long, default_value = "text")]
    pub output: String,
}