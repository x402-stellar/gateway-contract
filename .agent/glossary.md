# Glossary: x402-gateway-contract

- **x402**: Open internet standard using HTTP 402 "Payment Required" for programmatic M2M micropayments.
- **Soroban**: The smart contract platform on the Stellar network based on WebAssembly (WASM) and Rust.
- **SAC (Stellar Asset Contract)**: Native smart contract representation of classic Stellar assets (XLM, USDC).
- **Auth Entry (`SorobanAuthorizationEntry`)**: Cryptographically signed payload by an account authorizing specific contract invocations without submitting a full envelope.
- **TTL (Time-to-Live)**: Rent lifecycle threshold for ledger entries preventing state bloat.
- **Basis Points (bps)**: Percentage measurement where `100 bps = 1.00%` and `10_000 bps = 100.00%`.
- **Replay Protection**: Cryptographic and algorithmic mechanism ensuring an authorization payload cannot be executed twice.
