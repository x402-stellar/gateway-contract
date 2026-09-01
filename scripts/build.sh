#!/usr/bin/env bash
set -euo pipefail

echo "==> Building settlement-verifier contract for wasm32v1-none..."
stellar contract build

echo "==> Build successful! Artifact located in target/wasm32v1-none/release/"
