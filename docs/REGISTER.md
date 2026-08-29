# Owner-wallet registration

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

## Sign from the owner wallet

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

Using the connected browser wallet is preferable. After signing, record the transaction hash and emitted registration ID in `dist/registration.json`, then watch Telegraph's evaluator status before making any promotion claim.
