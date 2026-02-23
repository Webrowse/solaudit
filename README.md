# solaudit

A Solana transaction retry-safety analyzer. Fetches an on-chain account snapshot, optionally simulates a transaction, diffs the before/after state, and classifies whether retrying the transaction is safe.

## Features

- Account snapshot diffing (lamports, owner, executable, data size)
- Retry-safety classification with human-readable reasons
- RPC `simulateTransaction` integration for post-state analysis
- Text and JSON output formats

## Install

```bash
cargo build --release
```

The binary is at `target/release/solaudit`.

## Usage

### Snapshot only (no transaction)

Fetch the current state of an account and report it:

```bash
solaudit --program <ACCOUNT_PUBKEY> --cluster devnet
```

### With transaction simulation

Provide a base64-encoded transaction to simulate and diff:

```bash
solaudit --program <ACCOUNT_PUBKEY> --tx <BASE64_TX> --cluster devnet
```

### JSON output

Add `--output json` for machine-readable output:

```bash
solaudit --program <ACCOUNT_PUBKEY> --cluster devnet --output json
```

### CLI flags

| Flag | Description | Default |
|------|-------------|---------|
| `--program` | Account pubkey to track | required |
| `--cluster` | RPC cluster (`devnet`, `mainnet`) | `devnet` |
| `--tx` | Base64-encoded transaction to simulate | none |
| `--output` | Output format (`text`, `json`) | `text` |

## Running Tests & Demo

```bash
bash src/scripts/test.sh
```

This builds the project, runs `cargo test`, and does a live demo against devnet using the SysvarRent account.

## Project Structure

```
src/
  main.rs              Entry point
  cli/args.rs           CLI argument parsing (clap)
  models/types.rs       AccountSnapshot type
  analysis/engine.rs    Diffing and retry-safety classification
  report/writer.rs      Text and JSON output
  rpc/client.rs         Solana RPC wrapper
  scripts/test.sh       Build, test, and demo script
```

## Limitations

- Uses RPC simulation (not local runtime)
- Tracks a single account per run
- No CPI tracing

## Future Work

- Multi-account diff
- Surfpool local simulation backend
- CPI trace analysis
