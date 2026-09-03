# Release v0.1.0 - Settlement Verifier Smart Contract

**Release Tag**: `v0.1.0`  
**Network**: Stellar Testnet  
**Contract Address**: [`CATZACNU6KVGZXYF7J4O4NLINRKL5FWC2YAQPHTIQMSQPDAJSSOMRUNL`](https://stellar.expert/explorer/testnet/contract/CATZACNU6KVGZXYF7J4O4NLINRKL5FWC2YAQPHTIQMSQPDAJSSOMRUNL)  
**WASM Hash**: `1bd873a82c8359842cea41c2fb2ee00ca9e779938708458d090cec50e7818218`  
**Documentation**: [https://x402-stellar.mintlify.app](https://x402-stellar.mintlify.app)

---

## What's Changed in v0.1.0

### On-Chain Settlement Verification
* Deployed and initialized the `settlement-verifier` contract on Stellar Testnet.
* Verified signed Soroban authorization entries and enforced monotonic nonces per payer to completely prevent transaction replay attacks.
* Atomic multi-recipient revenue splitting with configurable protocol fee basis points (25 bps).

### Resource & Gas Optimization
* Resource consumption benchmarks implemented with Soroban's budget estimation API (`contracts/settlement-verifier/tests/benchmarks.rs`).
* `settle_payment` consumes **514,163 CPU instructions** and **156,208 memory bytes** (less than 0.52% of the network 100M limit).
* Documented methodology in `benchmarks.md`.

### Governance & Contributor Infrastructure
* Structured issue forms (`bug_report.yml`, `feature_request.yml`) and pull request template.
* Apache-2.0 license compliance and Rustfmt/Clippy clean verification.
