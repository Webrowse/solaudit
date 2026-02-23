#!/bin/bash
set -e

echo "=== solaudit test & demo ==="

# 1. Build
echo ""
echo "[1/3] Building..."
cargo build
echo "Build OK"

# 2. Unit tests
echo ""
echo "[2/3] Running unit tests..."
cargo test
echo "Tests OK"

# 3. Live demo against devnet (read-only, no tx)
echo ""
echo "[3/3] Running demo (devnet snapshot of SysvarRent)..."
echo ""
echo "--- Text output ---"
cargo run -- \
  --program SysvarRent111111111111111111111111111111111 \
  --cluster devnet

echo ""
echo "--- JSON output ---"
cargo run -- \
  --program SysvarRent111111111111111111111111111111111 \
  --cluster devnet \
  --output json

echo ""
echo "=== All checks passed ==="
