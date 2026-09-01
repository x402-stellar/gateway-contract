# x402-gateway-contract

[![CI](https://github.com/stellar-x402/x402-gateway-contract/actions/workflows/ci.yml/badge.svg)](https://github.com/stellar-x402/x402-gateway-contract/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)

`x402-gateway-contract` is a high-performance Soroban smart contract providing trustless cryptographic settlement verification, multi-party revenue splitting, and replay-attack protection for x402 HTTP micropayments on the Stellar network. When AI agents and automated clients settle per-request API paywalls using Stellar Asset Contracts (SAC), `x402-gateway-contract` verifies invocation authorization entries, distributes protocol and merchant fees atomically, tracks nonce lifecycles, and emits verifiable settlement receipts on-chain. It serves as the decentralized settlement backbone for the Stellar x402 Gateway reverse-proxy and middleware ecosystem.

## Contract Architecture

```
Client / Agent ──(x402 payment authorization)──▶ x402-gateway-contract
                                                          │
                    ┌─────────────────────────────────────┴─────────────────────────────────────┐
                    ▼                                                                           ▼
       Settlement Verification                                                       Revenue Distribution
  - Verifies monotonic nonce per payer                                           - Transfers net amount to merchant
  - Enforces valid token & positive amount                                       - Collects protocol fee in basis points
  - Emits on-chain SettlementReceipt event                                       - Supports multi-recipient splits
```

## Quickstart

### Prerequisites
- Rust 1.84.0 (`rustup default 1.84.0`)
- `wasm32v1-none` target (`rustup target add wasm32v1-none`)
- `stellar-cli` 27.1.0 (`cargo install stellar-cli --locked`)

### Build
```bash
bash scripts/build.sh
```

### Test
```bash
cargo test
```

## Contributing
See [CONTRIBUTING.md](CONTRIBUTING.md) for code standards and commit conventions.

## Security
See [SECURITY.md](SECURITY.md) for vulnerability reporting.
