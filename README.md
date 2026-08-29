# WEATHERPROOF

**Score the forecast facts, not the prose.**

WEATHERPROOF is a deterministic, zero-import Rust/WASM scoring module for Telegraph's `WEATHER_FORECAST` intent. It compares the facts that make a forecast useful: time window, conditions, measurements, units, coverage, and contradictions.

A generic similarity scorer can reward an answer that repeats the right words while changing `31°C` to `51°C`. WEATHERPROOF makes that one flipped fact collapse the score.

## Recorded result

The committed 15-case adversarial corpus compares the release binary against Telegraph registration `#636`, the live incumbent captured on 2026-08-29.

| Metric | WEATHERPROOF | Incumbent #636 |
|---|---:|---:|
| Correct beats corrupted | 15/15 | 12/15 |
| Correct beats gaming attack | 15/15 | 4/15 |
| Average correct–corrupt margin | **0.9923** | 0.6588 |
| Average correct–attack margin | **0.9827** | -0.0661 |
| Imports | **0** | 0 |
| Release size | **16,957 bytes** | — |

These are reproducible local results, not claims about Telegraph's hidden evaluation. Promotion can only be established after live registration.

## What it understands

- Celsius and Fahrenheit
- km/h, mph, and m/s
- millimetres and inches
- precipitation probability
- high, low, peak, chance, and threshold roles
- clear, cloud, rain, storm, snow, fog, wind, hail, heat, cold, and dry conditions
- today/tomorrow, weekdays, and dayparts
- omissions, contradictions, irrelevant padding, and token stuffing

The scorer is deliberately small and deterministic. It makes no network calls, imports no host functions, and keeps all results finite and bounded to `[0,1]`.

## Build and verify

Requirements: Rust stable with `wasm32-unknown-unknown`, and Node.js 18+.

```bash
rustup target add wasm32-unknown-unknown
./scripts/build.sh
node scripts/demo.mjs
node scripts/verify.mjs
```

`verify.mjs` fetches the immutable incumbent binary referenced by the corpus, validates the ABI, runs 15 paired cases, checks blank/self/unicode/128 KiB behavior, and performs 500 deterministic fuzz trials.

## Telegraph ABI

The artifact exports:

- `memory`
- `alloc`
- `dealloc`
- `rank_answer`
- `breakdown_answer`

`rank_answer(question, ground_truth, miner_answer) -> f32` returns the final score. `breakdown_answer` exposes five `f32` values for debugging: relevance, factual correctness, lexical overlap, length quality, and composite.

## Registration

The immutable binary and exact registration values are recorded in [`dist/registration.json`](dist/registration.json). Register the module under the canonical intent `WEATHER_FORECAST` on Telegraph's Base Sepolia Diamond.

## Evidence and limits

- [`PRD.md`](PRD.md) defines scope and success gates.
- [`TASKS.md`](TASKS.md) records execution status.
- [`docs/SCOUT.md`](docs/SCOUT.md) records the competitive scout and sources.
- [`docs/SUBMISSION.md`](docs/SUBMISSION.md) contains the submission draft.

WEATHERPROOF judges an answer against supplied ground truth. It does not independently fetch or attest weather observations. See the PRD for the complete limitations.

MIT licensed.
