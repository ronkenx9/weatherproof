# Owner-wallet registration

## Registration #1849 result

- Registration ID: `1849`
- Registrant: `0xc721912d00c84003015f01c1051a639092603923`
- Transaction: [`0x5d38…2344`](https://sepolia.basescan.org/tx/0x5d38cb61f1e895decc2f8aa3d808adfa8ad02ee3f292eefa578a96413f002344)
- Telegraph IPFS URL: `https://gateway.pinata.cloud/ipfs/Qmf9R5id7EaNphjE74BuUU8kV15v9Bk9nW5W9FHJsUsGDo`
- Evaluation: `rejected`
- Candidate wins: `15/15`; champion wins: `15/15`
- Candidate margin: `0.76872617`; champion margin: `0.9905663`
- Worst self-match: `1`; score standard deviation: `0.48028797`

The scorer ordered every hidden pair correctly. It lost only because the final scores were not polarized enough.

## v0.4 — final bounded retry

v0.2 (`1860`) and v0.3 (`1873`) were rejected on the same hidden ordering miss (`14/15`). v0.4 is the final bounded retry.

- Transaction: [`0xe313…450c`](https://sepolia.basescan.org/tx/0xe3135a877abf8cb9a720684bca6a62fa5dd067b93901ace59867268a9f45450c)
- Telegraph IPFS URL: `https://gateway.pinata.cloud/ipfs/QmaQHw6ZRnQiByDYiPRm5cqJQ7apbALiXT2TDFZRq6epDe`

Do not register it again. Watch the Dashboard for the Stage 2 result.

```text
Hash:     0x316aa104de22e9584e7cf586b1c2ce079a5cd083382efa16477e9c38bd6b91d0
Intent:   WEATHER_FORECAST
URL:      https://raw.githubusercontent.com/ronkenx9/weatherproof/278f27693a197ce758ae0194ceea0068d746f3e0/dist/weatherproof.wasm
Bytes:    17712
```

```text
Hash:     0xb749d778a4e53906381c805a41388b0d2342306195816b2a7bb81400d50da2e7
Intent:   WEATHER_FORECAST
URL:      https://raw.githubusercontent.com/ronkenx9/weatherproof/a61f43ab2084a5d173c463127637a5891ce63eb7/dist/weatherproof.wasm
Bytes:    17125
```

Local v0.2 separation is `0.999999996` across the ordinary corpus and `0.999989127` across attacks. The calibration is monotonic, so it does not change v0.1's pairwise ordering.

All non-signing checks were completed on Base Sepolia on 2026-08-29:

- RPC chain ID: `84532`
- Telegraph Diamond has deployed code.
- Exact `registerWasm(bytes32,string,string)` call succeeds under `eth_call`.
- Simulated return value: `0x72b` (registration ID `1835` if no earlier transaction consumes it).
- Gas estimate: `354108`.

## v0.1 historical values

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
