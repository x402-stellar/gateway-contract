# gateway-contract

[![CI](https://github.com/x402-stellar/gateway-contract/actions/workflows/ci.yml/badge.svg)](https://github.com/x402-stellar/gateway-contract/actions/workflows/ci.yml)
[![Docs](https://img.shields.io/badge/docs-mintlify-blue.svg)](https://github.com/x402-stellar/docs)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)

x402-gateway-contract handles on-chain settlement and fee distribution for HTTP 402 payments on Stellar. When a client or AI agent pays for an API call using a Stellar Asset Contract (SAC), this contract checks the signed authorization entry, splits payments between merchants and fee recipients in a single transaction, tracks monotonic nonces to prevent replay attacks, and emits structured receipt events to the ledger. It gives API gateways an on-chain verification point without requiring custom contract code for every merchant.

## Architecture

```
Client / Agent ---> (Payment Authorization) ---> gateway-contract
                                                       |
                    +----------------------------------+----------------------------------+
                    |                                                                     |
                    v                                                                     v
       Settlement Verification                                               Revenue Distribution
  - Verifies monotonic nonce per payer                                   - Transfers net amount to merchant
  - Enforces valid token & positive amount                               - Collects protocol fee in basis points
  - Emits on-chain SettlementReceipt event                               - Supports multi-recipient splits
```

## Quickstart

### Prerequisites
- Rust 1.92.0 (`rustup default 1.92.0`)
- `wasm32v1-none` target (`rustup target add wasm32v1-none`)
- `stellar-cli` 27.1.0 or newer

### Build
```bash
bash scripts/build.sh
```

### Test
```bash
cargo test
```

## Contributing
See [CONTRIBUTING.md](CONTRIBUTING.md) for commit standards and development guidelines.

## Security
See [SECURITY.md](SECURITY.md) for vulnerability reporting.
