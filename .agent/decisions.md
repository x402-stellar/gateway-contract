# Architecture Decision Records: x402-gateway-contract

Append-only. Never rewrite an entry. If a decision is reversed, append a new ADR that supersedes the old one.

---

## ADR-001: Adopt soroban-sdk 26.1.0 stable

Date: 2026-09-01
Status: accepted

### Context
Soroban SDK 26.1.0 is the current stable release on Stellar Mainnet and Testnet. Version 27 release candidates (27.0.0-rc.x) introduce experimental changes not yet universally deployed across public validator nodes.

### Decision
Pin `soroban-sdk = "26.1.0"` in `contracts/settlement-verifier/Cargo.toml`.

### Alternatives considered
- **`27.0.0-rc.2`**: Rejected due to pre-release API volatility.
- **`25.x`**: Rejected as it lacks current TTL management macros and `#[contractevent]`.

### Consequences
- Build target is strictly `wasm32v1-none`.
- Compiles reliably with Rust 1.84.0.

---

## ADR-002: Target wasm32v1-none exclusively

Date: 2026-09-01
Status: accepted

### Context
On modern Rust toolchains (1.82+), the legacy `wasm32-unknown-unknown` target creates incompatibility with Soroban runtime memory layouts and wasm-opt passes.

### Decision
Pin `targets = ["wasm32v1-none"]` in `rust-toolchain.toml`. Build contract artifacts using `stellar contract build`.

### Consequences
- Standard `cargo build --target wasm32-unknown-unknown` is banned.
- All developer workflows must invoke `stellar contract build` or `scripts/build.sh`.

---

## ADR-003: Monotonic Per-Account Nonces for Replay Protection

Date: 2026-09-01
Status: accepted

### Context
In high-frequency x402 API billing, clients sign Soroban authorization entries. If an adversary captures a raw signed entry from network traffic or server logs, they could attempt to replay it against the contract to drain funds or cause duplicate payment settlement.

### Decision
Maintain a persistent `Nonce(Address) -> u64` in contract storage. Any settlement call verifying a payment for `payer` requires `nonce == current_nonce + 1`, and atomically increments `current_nonce`.

### Alternatives considered
- **Timestamp / Ledger Expiry only**: Leaves a window of vulnerability during the validity duration.
- **Random UUID Nonces (HashSet)**: Storage grows indefinitely without predictable bounds.

### Consequences
- Replay is mathematically impossible.
- Out-of-order execution from the same payer must be sequenced sequentially by the client SDK.

---

## ADR-004: Basis Points Math for Multi-Recipient Payment Splits

Date: 2026-09-01
Status: accepted

### Context
API gateways frequently need to split incoming payments between multiple upstream parties (e.g., API Provider, Model Host, Gateway Operator). Blockchain smart contracts forbid floating-point operations.

### Decision
Represent all percentage splits as Basis Points (`u32`), where `10_000 bps = 100.00%`. Enforce that the sum of all split fractions plus the protocol fee equals exactly `10_000` (or `amount` remainder is credited to primary merchant).

### Consequences
- Zero floating-point rounding errors.
- Precise arithmetic using `(amount * bps as i128) / 10_000`.

---

## ADR-005: Event-Driven Receipt Emission via #[contractevent]

Date: 2026-09-01
Status: accepted

### Context
Off-chain indexers and gateways need immediate, structured proof that a payment settled without polling full ledger state changes.

### Decision
Define explicit event structs with `#[contractevent]` in `events.rs`: `SettlementReceipt` (topics: `["settle", merchant, payer]`, data: `(token, amount, fee, nonce)`) and `SplitReceipt`.

### Consequences
- Indexers can filter events by merchant or payer directly using Soroban RPC topic filters.
- Receipts are permanently anchored in ledger history.
