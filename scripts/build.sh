#!/usr/bin/env bash
set -euo pipefail

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_DIR"

cargo build --release --target wasm32-unknown-unknown
mkdir -p dist
cp target/wasm32-unknown-unknown/release/weatherproof_scorer.wasm dist/weatherproof.wasm

printf 'built dist/weatherproof.wasm (%s bytes)\n' "$(wc -c < dist/weatherproof.wasm | tr -d ' ')"
