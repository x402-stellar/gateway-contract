# Context: x402-gateway-contract

Onboarding for a fresh session, AI or human. Read this before touching code. Updated at phase transitions, not at every commit.

## 1. What x402-gateway-contract is

`x402-gateway-contract` is the on-chain Soroban contract layer for the Stellar x402 Gateway. While the x402 protocol allows servers to challenge clients with HTTP 402 responses and clients to provide signed payment payloads, having the merchant's API directly accept off-chain transfers introduces two vulnerabilities:
1. **Replay Attacks**: A captured authorization payload could be replayed against other endpoints or multiple times unless strictly bound by an on-chain monotonic nonce or timestamp gate.
2. **Atomic Revenue Splitting**: Merchants often need to split incoming payments between multiple stakeholders (e.g. 80% to model creator, 10% to compute provider, 10% to gateway facilitator). Doing multiple individual transfers costs extra round-trips and ledger fees.

This contract solves both issues in a single atomic transaction.

## 2. Why this repo exists

`x402-gateway-contract` handles the critical cryptographic and financial logic on Stellar. The companion repository `x402-gateway-app` consumes this contract via TypeScript SDK bindings and Go proxy clients.

## 3. Relationship to x402-gateway-app

- `x402-gateway-contract` exports contract WASM and generated TypeScript/Go client bindings.
- Once deployed to Testnet, its Contract ID is pinned in `x402-gateway-app`'s `.env.example` as `STELLAR_X402_SETTLEMENT_CONTRACT_ID`.
- If the off-chain middleware and on-chain contract disagree on the transaction signature or fee math, the contract is authoritative.

## 4. Architecture & Core Invariants

- **Storage**:
  - `Instance` storage for `Admin`, `FeeBps`, `FeeRecipient`, and `Initialized`.
  - `Persistent` storage for `Nonce(Address)` tracking with automated TTL extension on every write.
- **Authorization**:
  - All payment settlements require `payer.require_auth()` via Soroban auth entry.
  - Admin configuration functions require `current_admin.require_auth()`.
- **Numbers**:
  - All token values are stored and transferred as `i128`. No floating-point math anywhere.
  - Fee calculations use basis points (`u32`, `10_000 = 100%`).
- **Replay Protection**:
  - Each settlement call must supply a `nonce` strictly equal to `current_nonce + 1`.

## 5. Drips Wave & Maintainer Context

This repository is designed for approval in the Stellar Drips Wave program and the Stellar Community Fund (SCF). Every function has 100% unit test coverage, zero compiler warnings under `clippy::pedantic`, explicit documentation, and standardized CI workflows.
