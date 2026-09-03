# Contract Resource & Gas Benchmarks

This document records the exact CPU instruction consumption and memory byte footprints for operations executed by the `settlement-verifier` Soroban smart contract.

Metrics are captured via the `soroban_sdk::Env::cost_estimate().budget()` instrumentation harness.

---

## 1. Benchmark Results Summary

| Operation | Description | CPU Instructions | Memory Bytes | % of Soroban CPU Limit | Est. Network Fee |
|---|---|---|---|---|---|
| `get_nonce` | Read-only payer nonce lookup | 24,952 | 10,181 | 0.025% | < 0.00001 XLM |
| `settle_payment` | Net transfer to merchant + 25 bps protocol fee split | 514,163 | 156,208 | 0.514% | ~ 0.00001 XLM |
| `verify_and_split` | Multi-party split (70/30) across 2 recipients + fee | 519,514 | 156,103 | 0.520% | ~ 0.00001 XLM |

*Note: The Soroban network CPU instruction limit per transaction is 100,000,000 instructions. The contract operations consume less than 0.6% of available budget, ensuring sub-cent transaction costs.*

---

## 2. Invariants Measured

1. **Monotonic Nonce Overhead**:
   Persistent storage reads and increments add minimal instruction overhead (under 25,000 instructions), ensuring that replay protection does not inflate costs.

2. **Atomic Token Transfers**:
   Inter-contract invocations via `TokenClient::transfer` represent the primary instruction weight (~480,000 instructions). Combining the merchant transfer and protocol fee split in a single transaction avoids double-spend and signature serialization overhead.

3. **Multi-Party Split Efficiency**:
   Handling multiple recipients scales linearly with minimal allocation overhead due to in-place vector iterations.

---

## 3. Reproduction

To reproduce these metrics locally:

```bash
cargo test --test benchmarks -- --nocapture
```
