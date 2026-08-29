# Telegraph Track 2 competitive scout

Captured on 2026-08-29. Counts are a point-in-time snapshot and may change as authors register new modules.

## Rules that shaped the build

Telegraph Track 2 is for WASM script authors. The protocol's evaluator first checks the runtime contract, then compares a candidate against the current champion using paired ranking accuracy and separation. The module must be deterministic, freestanding, bounded, and registered under an exact canonical intent.

Primary sources:

- [Hackathon rules](https://hackathon.telegraphprotocol.com/rules)
- [Build a scoring module](https://github.com/telegraphprotocol/telegraph-docs/blob/main/scoring/build-a-scoring-module.md)
- [Telegraph WASM validation tool](https://github.com/nerom/telegraph-wasm-check)
- [Live miner integrations](http://13.237.89.59:7044/miner-dispatcher/integrations)
- Base Sepolia Diamond: [`0x5a2324aA18613FAD4e44bDF0d6c73Ec1f6D87ff8`](https://sepolia.basescan.org/address/0x5a2324aA18613FAD4e44bDF0d6c73Ec1f6D87ff8)

## Network snapshot

- 106 miner integrations exposed by the live dispatcher.
- 1,833 indexed WASM registry records.
- 1,784 active WASM entries from 44 distinct authors.
- 45 active champions across canonical intents.
- Status distribution: 1,574 rejected, 43 deregistered, 164 superseded, 45 champions, 1 pending.
- One address accounts for 847 scripts spanning all 45 intents, so raw script count overstates genuine competition.

## Why WEATHER_FORECAST

`WEATHER_FORECAST` had 78 registered scripts, 10 distinct authors, 14 declaring miners, and 12 ranked miners. Its incumbent was credible but still left a clear domain-specific opening:

- registration: `#636`
- pairwise wins: 15/15
- recorded margin: `0.53020585`
- historical rows: 21
- Spearman agreement: `0.81260157`
- immutable artifact: [`wf_mini.wasm`](https://raw.githubusercontent.com/zkasuran/telegraph-salience-scorer/f009d2d778bd49611dcc0a7e3819a8dca74d1aad/dist/xfmr/wf_mini.wasm)

The incumbent is a generic MiniLM/salience scorer deployed across many intents. Its semantic strength is also the exploit: high-overlap answers can preserve almost the same score after changing a critical value, unit, time window, or condition.

Live registration `#1849` later showed that the incumbent reached a `0.9905663` margin on the current 15 hidden fixtures. WEATHERPROOF v0.1 also won 15/15 pairs but scored a `0.76872617` margin, so v0.2 adds monotonic confidence calibration without changing the underlying ordering.

## Alternatives rejected

- `ONCHAIN_TX_LOOKUP`: strong product relevance, but incumbent margin was already about `0.792`.
- `FRAUD_DETECTION`: 213 registered scripts and an incumbent margin around `0.878`; crowded and strong.
- `CVE_LOOKUP`: incumbent margin was effectively saturated near `1.0`.
- Generic cross-intent semantic scorer: already heavily represented and easy to game with fluent contradiction.

Existing local code was not counted as an advantage. The target was selected solely on problem quality, competitive gap, demonstrability, and benchmark fit.

## Winning wedge

The demo changes one decision-critical fact while keeping almost all the prose:

```text
truth:    clear and dry, 31°C, winds 18 km/h
attack:   clear and dry, 51°C, winds 18 km/h
```

The semantic surface barely changes. The forecast does. WEATHERPROOF is built around that distinction.
