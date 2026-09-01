#!/usr/bin/env bash
set -euo pipefail

NETWORK="testnet"
RPC_URL="https://soroban-testnet.stellar.org"
NETWORK_PASSPHRASE="Test SDF Network ; September 2015"
SOURCE_ACCOUNT="${DEPLOYER_SECRET:-deployer}"

echo "==> Deploying settlement-verifier to Stellar $NETWORK..."

WASM_PATH="target/wasm32v1-none/release/settlement_verifier.wasm"

if [ ! -f "$WASM_PATH" ]; then
    echo "WASM not found. Running build.sh first..."
    bash scripts/build.sh
fi

CONTRACT_ID=$(stellar contract deploy \
    --wasm "$WASM_PATH" \
    --source "$SOURCE_ACCOUNT" \
    --network "$NETWORK")

echo "=================================================="
echo "Contract deployed successfully!"
echo "CONTRACT_ID: $CONTRACT_ID"
echo "Network: $NETWORK"
echo "=================================================="
