## Description
Briefly describe the change and the problem it solves. Reference any related issues using `Fixes #` or `Closes #`.

---

## Type of Change
- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Gas optimization (improving CPU/memory resource usage)
- [ ] Breaking change (fix or feature that changes existing entry point interfaces)
- [ ] Documentation update

---

## Verification Checklist
- [ ] `cargo fmt --all -- --check` passes with zero diffs.
- [ ] `cargo clippy --all-targets -- -D warnings` passes with zero warnings.
- [ ] `cargo test --all-targets` passes all tests.
- [ ] Added unit tests covering all new logic or error branches.
- [ ] If contract logic changed, ran `cargo test --test benchmarks -- --nocapture` and updated `benchmarks.md`.
- [ ] Zero floating-point arithmetic; integer arithmetic bounded to prevent overflow.
