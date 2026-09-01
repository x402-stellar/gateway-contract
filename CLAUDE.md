# CLAUDE.md: x402-gateway-contract

**Project**: `x402-gateway-contract`
**Role**: Soroban smart contract layer for x402 settlement verification and fee distribution
**Sibling Repo**: `github.com/stellar-x402/x402-gateway-app`
**Current Phase**: Phase 6 (Contract Implementation)

## Non-Negotiables
- Target: `wasm32v1-none`, build via `stellar contract build`
- Version: `soroban-sdk = "26.1.0"` pinned in `Cargo.toml`
- Rust toolchain: `1.84.0` pinned in `rust-toolchain.toml`
- No `unwrap()`, `expect()`, or `panic!` in contract source code
- Amounts: strictly `i128` (no floats anywhere)
- Commits: one logical unit per commit, push immediately, never `git add .`

## Authoritative Documentation
- System Prompt: `docs/planning/system-prompt.md`
- Context & Invariants: `.agent/context.md`
- Decision Log: `.agent/decisions.md`
