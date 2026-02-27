# solaudit

[![Crates.io](https://img.shields.io/crates/v/solaudit.svg)](https://crates.io/crates/solaudit)
[![Docs.rs](https://docs.rs/solaudit/badge.svg)](https://docs.rs/solaudit)
[![License:
MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/Webrowse/solaudit/actions/workflows/ci.yml/badge.svg)](https://github.com/Webrowse/solaudit/actions)

A CLI tool for analyzing retry-safety of Solana transactions via
deterministic state diffing.

`solaudit` simulates transactions, captures on-chain account state
before and after execution, computes state diffs, and classifies whether
retrying a transaction is safe or unsafe.

Once a Solana transaction mutates persistent state, retrying it may lead
to duplicated or irreversible effects. This tool makes those state
changes explicit before submission.

------------------------------------------------------------------------

## Quick Install

Install via Cargo:

``` bash
cargo install solaudit
```

Or build from source:

``` bash
git clone https://github.com/Webrowse/solaudit
cd solaudit
cargo build --release
```

------------------------------------------------------------------------

## Motivation

On Solana, failed or ambiguous transactions often leave developers
unsure whether an instruction partially executed.

Today, developers rely on logs and manual inspection to determine retry
safety. This is slow, error-prone, and unreliable for complex workflows.

`solaudit` provides:

-   Explicit before/after state snapshots\
-   Deterministic state diffing\
-   Automated retry-safety classification\
-   Structured JSON output for CI

------------------------------------------------------------------------

## Core Features

-   Persistent account snapshot capture\
-   Deterministic diff engine (lamports, owner, executable flag, data
    size)\
-   Retry-safety classification with explanations\
-   RPC `simulateTransaction` integration\
-   Human-readable and JSON output formats\
-   Anchor-compatible workflow\
-   15 unit tests covering classification and diff logic

------------------------------------------------------------------------

## Usage

### Snapshot Only (No Transaction)

``` bash
solaudit --program <ACCOUNT_PUBKEY> --cluster devnet
```

### With Transaction Simulation

``` bash
solaudit   --program <ACCOUNT_PUBKEY>   --tx <BASE64_TX>   --cluster devnet
```

### JSON Output (CI / Automation)

``` bash
solaudit   --program <ACCOUNT_PUBKEY>   --tx <BASE64_TX>   --output json
```

Exit codes can be used for CI enforcement workflows.

------------------------------------------------------------------------

## CLI Flags

  Flag          Description                         Default
  ------------- ----------------------------------- ----------
  `--program`   Account pubkey to monitor           required
  `--cluster`   RPC cluster (`devnet`, `mainnet`)   `devnet`
  `--tx`        Base64 transaction to simulate      none
  `--output`    Output format (`text`, `json`)      `text`

Note: `--program` specifies the account being monitored, not the program
ID.

------------------------------------------------------------------------

## Architecture Overview

    CLI Input
       ↓
    RPC Snapshot (Pre-state)
       ↓
    Transaction Simulation
       ↓
    RPC Snapshot (Post-state)
       ↓
    State Diff Engine
       ↓
    Retry-Safety Classification
       ↓
    Report Generation

Each layer is modularized and tested independently.

------------------------------------------------------------------------

## Integration with Anchor

Typical workflow:

1.  Generate transaction in Anchor client\
2.  Serialize to base64\
3.  Analyze with `solaudit`\
4.  Decide whether to submit

Example:

``` js
const tx = await program.methods.deposit(...).transaction();

const serialized = tx.serialize({
  verifySignatures: false,
  requireAllSignatures: false,
});

const base64 = Buffer.from(serialized).toString("base64");
```

Then:

``` bash
solaudit --program <PDA> --tx "<base64>"
```

------------------------------------------------------------------------

## Project Structure

    src/
      main.rs              CLI orchestration
      cli/args.rs          CLI parsing
      models/types.rs      AccountSnapshot model
      analysis/engine.rs   Diff + classification engine
      report/writer.rs     Text / JSON reporting
      rpc/client.rs        Solana RPC integration
      scripts/test.sh      Smoke test

------------------------------------------------------------------------

## Limitations

-   Uses public RPC simulation (not full validator execution)\
-   Tracks one account per run\
-   No CPI-level tracing\
-   Simulation behavior may differ from on-chain execution

------------------------------------------------------------------------

## Future Work

-   Multi-account diffing\
-   Local execution backend (Surfpool / LiteSVM)\
-   CPI call tracing\
-   Workflow-level transaction analysis\
-   Enhanced automation support

------------------------------------------------------------------------

## License

MIT
