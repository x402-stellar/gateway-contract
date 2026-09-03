# gateway-contract

[![CI](https://github.com/x402-stellar/gateway-contract/actions/workflows/ci.yml/badge.svg)](https://github.com/x402-stellar/gateway-contract/actions/workflows/ci.yml)
[![Docs](https://img.shields.io/badge/docs-x402--stellar.mintlify.app-blue.svg)](https://x402-stellar.mintlify.app)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)

x402-gateway-contract handles on-chain settlement and fee distribution for HTTP 402 payments on Stellar. When a client or AI agent pays for an API call using a Stellar Asset Contract (SAC), this contract checks the signed authorization entry, splits payments between merchants and fee recipients in a single transaction, tracks monotonic nonces to prevent replay attacks, and emits structured receipt events to the ledger. It gives API gateways an on-chain verification point without requiring custom contract code for every merchant.

Documentation and API reference: [https://x402-stellar.mintlify.app](https://x402-stellar.mintlify.app)

---

## Testnet Deployment

The `settlement-verifier` contract is deployed and active on Stellar Testnet:

| Parameter | Value |
|---|---|
| **Contract ID** | [`CATZACNU6KVGZXYF7J4O4NLINRKL5FWC2YAQPHTIQMSQPDAJSSOMRUNL`](https://stellar.expert/explorer/testnet/contract/CATZACNU6KVGZXYF7J4O4NLINRKL5FWC2YAQPHTIQMSQPDAJSSOMRUNL) |
| **WASM Hash** | `1bd873a82c8359842cea41c2fb2ee00ca9e779938708458d090cec50e7818218` |
| **Admin Address** | `GCQURZFYPPAN76FRARROTSTYVH2LQ5AP7OLDXMJPIQ7STDOM55FXWD4T` |
| **Protocol Fee** | 25 BPS (0.25%) |
| **Network** | Stellar Testnet |

---

## Resource & Gas Benchmarks

Operations run well below the Soroban network limit of 100,000,000 CPU instructions per transaction:

| Operation | CPU Instructions | Memory Bytes | % of CPU Limit | Est. Fee |
|---|---|---|---|---|
| `get_nonce` | 24,952 | 10,181 | 0.025% | < 0.00001 XLM |
| `settle_payment` | 514,163 | 156,208 | 0.514% | ~ 0.00001 XLM |
| `verify_and_split` | 519,514 | 156,103 | 0.520% | ~ 0.00001 XLM |

For full benchmarking methodology and reproduction commands, see [benchmarks.md](benchmarks.md).

---

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

---

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
cargo test --all-targets
```

### Benchmarks
```bash
cargo test --test benchmarks -- --nocapture
```

---

## Contributing
See [CONTRIBUTING.md](CONTRIBUTING.md) for commit standards and development guidelines.

## Security
See [SECURITY.md](SECURITY.md) for vulnerability reporting.
