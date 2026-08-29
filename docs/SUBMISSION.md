# Telegraph Track 2 submission draft

## Project name

WEATHERPROOF

## Intent

`WEATHER_FORECAST`

## One-line description

A deterministic Telegraph WASM judge that scores forecast facts, units, time windows, and contradictions instead of rewarding prose similarity.

## Problem

Weather answers can look semantically identical while disagreeing on the one fact that changes a decision. A fluent answer that flips `31°C` to `51°C`, `tomorrow` to `today`, or `clear` to `thunderstorms` should not inherit a high score from shared vocabulary.

## Solution

WEATHERPROOF parses the supplied ground truth and miner answer into typed evidence. It normalizes temperature, wind, precipitation, probability, high/low/peak/threshold roles, temporal scope, and weather-condition polarity. Its composite rewards coverage and equivalence while sharply penalizing contradictions, missing evidence, and stuffing.

It compiles to a 17,125-byte freestanding WASM module with zero imports and the complete Telegraph ABI.

## Reproducible evidence

Against the immutable live incumbent `#636` on the committed 15-case corpus:

- WEATHERPROOF: 15/15 correct-over-corrupt wins; incumbent: 12/15.
- WEATHERPROOF: 15/15 correct-over-attack wins; incumbent: 4/15.
- Average ordinary separation: `0.999999996` versus `0.6588` on the committed local corpus.
- Average attack separation: `0.999989127` versus `-0.0661`.
- Zero candidate regressions across the 15 paired cases.
- 500 deterministic fuzz trials, Unicode input, blank input, and 128 KiB input pass without traps.

These are local, reproducible measurements. Live Telegraph promotion is reported only after the registry evaluator finishes.

## Demo flow

Run `./scripts/build.sh && node scripts/demo.mjs`. The terminal shows the correct forecast scoring near one, then the same sentence with a temperature flip, storm flip, and keyword-stuffing attack collapsing near zero. Run `node scripts/verify.mjs` for the full incumbent comparison.

## Links

- Repository: https://github.com/ronkenx9/weatherproof
- Immutable WASM: https://raw.githubusercontent.com/ronkenx9/weatherproof/a61f43ab2084a5d173c463127637a5891ce63eb7/dist/weatherproof.wasm
- Registration ID: `1860`
- Registration transaction: https://sepolia.basescan.org/tx/0xe3135a877abf8cb9a720684bca6a62fa5dd067b93901ace59867268a9f45450c
- v0.1 evaluation evidence: registration `1849`, rejected despite 15/15 ordering because its `0.7687` margin was below `0.9906`.
- X post: `OWNER_X_POST`

## Owner-only final actions

The owner must register the v0.2 hash, replace the registration placeholder above, publish the X post, and click the final hackathon submission action.
