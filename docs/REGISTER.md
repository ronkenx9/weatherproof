# Owner-wallet registration

## Live registration

- Registration ID: `1849`
- Registrant: `0xc721912d00c84003015f01c1051a639092603923`
- Transaction: [`0x5d38…2344`](https://sepolia.basescan.org/tx/0x5d38cb61f1e895decc2f8aa3d808adfa8ad02ee3f292eefa578a96413f002344)
- Telegraph IPFS URL: `https://gateway.pinata.cloud/ipfs/Qmf9R5id7EaNphjE74BuUU8kV15v9Bk9nW5W9FHJsUsGDo`
- Evaluation: `pending` as of 2026-08-30; do not claim promotion until Stage 2 reports.

All non-signing checks were completed on Base Sepolia on 2026-08-29:

- RPC chain ID: `84532`
- Telegraph Diamond has deployed code.
- Exact `registerWasm(bytes32,string,string)` call succeeds under `eth_call`.
- Simulated return value: `0x72b` (registration ID `1835` if no earlier transaction consumes it).
- Gas estimate: `354108`.

## Exact values

```text
Diamond:  0x5a2324aA18613FAD4e44bDF0d6c73Ec1f6D87ff8
Hash:     0x18145bb57172d9f53dc47e7e59180c5c036a4b071d469ae38635184081c5417a
Intent:   WEATHER_FORECAST
URL:      https://raw.githubusercontent.com/ronkenx9/weatherproof/7b2ab0ca5dde57fc7aaf6e3749e87eac65c7dad3/dist/weatherproof.wasm
```

## Reproduction command

Never paste a private key into chat or commit it. If the owner deliberately chooses Foundry, place the key in a local environment variable and run:

```bash
cast send 0x5a2324aA18613FAD4e44bDF0d6c73Ec1f6D87ff8 \
  'registerWasm(bytes32,string,string)' \
  0x18145bb57172d9f53dc47e7e59180c5c036a4b071d469ae38635184081c5417a \
  'https://raw.githubusercontent.com/ronkenx9/weatherproof/7b2ab0ca5dde57fc7aaf6e3749e87eac65c7dad3/dist/weatherproof.wasm' \
  'WEATHER_FORECAST' \
  --rpc-url https://sepolia.base.org \
  --private-key "$TELEGRAPH_PRIVATE_KEY"
```

The live registration was signed through Telegraph's integration portal. The command above is retained only as a reproducibility record; do not broadcast it again because duplicate binaries from the same address are refused.
