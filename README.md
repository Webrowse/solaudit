# solaudit

A Solana transaction retry-safety analyzer developed as a capstone project.

This tool simulates transactions, captures on-chain account state before and after execution, computes state diffs, and classifies whether retrying a transaction is safe or unsafe.

Once a Solana transaction mutates persistent state, retrying it may lead to duplicated or irreversible effects. This tool makes those state changes explicit before submission.

---

## Motivation

On Solana, failed or ambiguous transactions often leave developers unsure whether an instruction partially executed.

Today, developers rely on logs and manual inspection to determine retry safety. This is slow, error-prone, and unreliable for complex workflows.

`solaudit` provides:

- Explicit before/after state snapshots  
- Deterministic state diffing  
- Automated retry-safety classification  

---

## Core Features

- Persistent account snapshot capture  
- Before/after state diffing (lamports, owner, executable flag, data size)  
- Retry-safety classification with explanations  
- RPC `simulateTransaction` integration  
- Human-readable and JSON output formats  
- Anchor-compatible workflow  

---

## Architecture Overview

Execution pipeline:

```
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
```

Each layer is modularized in the codebase.

---

## Installation

Build from source:

```bash
cargo build --release
```

Binary output:

```
target/release/solaudit
```

---

## Usage

### 1. Snapshot Only (No Transaction)

Fetch and report the current state of an account:

```bash
solaudit --program <ACCOUNT_PUBKEY> --cluster devnet
```

---

### 2. With Transaction Simulation

Simulate a base64-encoded transaction and analyze state changes:

```bash
solaudit   --program <ACCOUNT_PUBKEY>   --tx <BASE64_TX>   --cluster devnet
```

---

### 3. JSON Output (CI / Automation)

Enable machine-readable output:

```bash
solaudit   --program <ACCOUNT_PUBKEY>   --cluster devnet   --output json
```

---

### CLI Flags

| Flag | Description | Default |
|------|-------------|---------|
| `--program` | Account pubkey to monitor | required |
| `--cluster` | RPC cluster (`devnet`, `mainnet`) | `devnet` |
| `--tx` | Base64 transaction to simulate | none |
| `--output` | Output format (`text`, `json`) | `text` |

Note: `--program` specifies the account being monitored, not the program ID.

---

## Demo & Smoke Test

Run:

```bash
bash scripts/test.sh
```

This performs:

- Project build  
- Live RPC connectivity check  
- Snapshot + classification on devnet  

For full demo, use a transaction generated from an Anchor project.

---

## Integration with Anchor

Typical workflow:

1. Generate transaction in Anchor client  
2. Serialize to base64  
3. Analyze with `solaudit`  
4. Decide whether to submit  

JavaScript example:

```js
const tx = await program.methods.deposit(...).transaction();

const serialized = tx.serialize({
  verifySignatures: false,
  requireAllSignatures: false,
});

const base64 = Buffer.from(serialized).toString("base64");
```

Then:

```bash
solaudit --program <PDA> --tx "<base64>"
```

---

## Project Structure

```
src/
  main.rs              CLI orchestration
  cli/args.rs          CLI parsing
  models/types.rs      AccountSnapshot model
  analysis/engine.rs   Diff + classification engine
  report/writer.rs     Text / JSON reporting
  rpc/client.rs        Solana RPC integration
  scripts/test.sh      Smoke test
```

---

## Limitations

- Uses public RPC simulation (not full validator execution)  
- Tracks one account per run  
- No CPI-level tracing  
- Simulation behavior may differ from on-chain execution  

These limitations are documented and considered acceptable for a proof-of-concept.

---

## Future Work

- Multi-account diffing  
- Local execution backend (Surfpool / LiteSVM)  
- CPI call tracing  
- Workflow-level transaction analysis  
- Enhanced automation support  

---

## Author

Developed as part of a capstone project focused on Solana transaction safety and workflow analysis.
