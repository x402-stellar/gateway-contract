# Deployment Record

All contract deployments, verification hashes, and network configurations are recorded here.

---

## 1. Stellar Testnet Deployment

- **Contract Name**: `settlement-verifier`
- **Network**: `stellar:testnet`
- **RPC URL**: `https://soroban-testnet.stellar.org`
- **Network Passphrase**: `Test SDF Network ; September 2015`
- **Contract ID**: `CATZACNU6KVGZXYF7J4O4NLINRKL5FWC2YAQPHTIQMSQPDAJSSOMRUNL`
- **WASM Hash**: `1bd873a82c8359842cea41c2fb2ee00ca9e779938708458d090cec50e7818218`
- **WASM Size**: 8,092 bytes
- **Deployer / Admin Address**: `GCQURZFYPPAN76FRARROTSTYVH2LQ5AP7OLDXMJPIQ7STDOM55FXWD4T`
- **Fee Recipient**: `GCQURZFYPPAN76FRARROTSTYVH2LQ5AP7OLDXMJPIQ7STDOM55FXWD4T`
- **Fee BPS**: 25 (0.25%)
- **Initialization Tx**: `2f9dd2bed52eb5256dc834a493510ed04756cb817d192739717745bbbaf106b3`
- **Explorer Link**: [StellarExpert Contract Page](https://stellar.expert/explorer/testnet/contract/CATZACNU6KVGZXYF7J4O4NLINRKL5FWC2YAQPHTIQMSQPDAJSSOMRUNL)
- **Deployment Timestamp**: `2026-09-03T13:29:17Z`

---

## 2. Soroban Contract Invocation Reference

To verify state via `stellar-cli`:
```bash
stellar contract invoke \
  --id CATZACNU6KVGZXYF7J4O4NLINRKL5FWC2YAQPHTIQMSQPDAJSSOMRUNL \
  --network testnet \
  --source deployer \
  -- \
  get_fee_config
```
