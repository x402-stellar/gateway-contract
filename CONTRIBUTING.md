# Contributing to x402-gateway-contract

We welcome contributions! Please review these guidelines before submitting pull requests.

## Git & Commit Conventions

We follow the **Conventional Commits** specification (`type(scope): description`):
- `feat(verifier): ...`
- `fix(verifier): ...`
- `test(verifier): ...`
- `docs: ...`
- `chore: ...`

Rules:
1. One logical unit per commit.
2. Never `git add .` — stage exact files.
3. Every public method must have corresponding unit tests.
4. Run `cargo test` and `cargo clippy -- -D warnings` before opening PRs.

## Code Standards
- Zero `unwrap()` / `expect()` / `panic!` in contract code (`contracts/**` outside tests).
- Zero floating-point arithmetic (strictly `i128` for token amounts).
- Enforce `require_auth()` on all state-changing entry points.
