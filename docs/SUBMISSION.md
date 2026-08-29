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

It compiles to a 16,957-byte freestanding WASM module with zero imports and the complete Telegraph ABI.

## Reproducible evidence

Against the immutable live incumbent `#636` on the committed 15-case corpus:

- WEATHERPROOF: 15/15 correct-over-corrupt wins; incumbent: 12/15.
- WEATHERPROOF: 15/15 correct-over-attack wins; incumbent: 4/15.
- Average ordinary separation: `0.9923` versus `0.6588`.
- Average attack separation: `0.9827` versus `-0.0661`.
- Zero candidate regressions across the 15 paired cases.
- 500 deterministic fuzz trials, Unicode input, blank input, and 128 KiB input pass without traps.

These are local, reproducible measurements. Live Telegraph promotion is reported only after the registry evaluator finishes.

## Demo flow

Run `./scripts/build.sh && node scripts/demo.mjs`. The terminal shows the correct forecast scoring near one, then the same sentence with a temperature flip, storm flip, and keyword-stuffing attack collapsing near zero. Run `node scripts/verify.mjs` for the full incumbent comparison.

## Links

- Repository: https://github.com/ronkenx9/weatherproof
- Immutable WASM: https://raw.githubusercontent.com/ronkenx9/weatherproof/7b2ab0ca5dde57fc7aaf6e3749e87eac65c7dad3/dist/weatherproof.wasm
- Registration transaction: `OWNER_REGISTRATION_TX`
- X post: `OWNER_X_POST`

## Owner-only final actions

The owner must connect the intended Base Sepolia wallet, sign the registration transaction, publish the X post, and click the final submission action.
