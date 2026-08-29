# WEATHERPROOF — Product Requirements

## One-line product

A deterministic Telegraph WASM judge that scores weather facts — place, time, units, values, and conditions — rather than rewarding prose similarity.

## Contest and deadline

- Telegraph Season I, Hackathon 1, Track 2: Script Authors
- Canonical intent: `WEATHER_FORECAST`
- Close: 2026-08-31 23:59:59 UTC
- Runtime: freestanding `wasm32-unknown-unknown`, no network/filesystem/state, <=32 MB

## Evidence for the target

Captured 2026-08-29 from Telegraph's live node and Base Sepolia registry:

- 106 live miner registrations across the network.
- 14 miners declare `WEATHER_FORECAST`; 12 are currently ranked.
- 1,784 active WASM entries from 44 distinct authors; experimentation is crowded but concentrated.
- Active `WEATHER_FORECAST` champion: registration `636`, benchmark margin `0.53020585`, 15/15 wins, 21 historical rows, Spearman `0.81260157`.
- All current live forecast-miner scores are below `0.01`.

The opportunity is not “write another similarity metric.” It is to make structured forecast correctness legible to Telegraph's quality flywheel.

## User and job

The direct user is a Telegraph validator. Given a question, a ground-truth answer, and a miner answer, it needs a bounded deterministic score that reflects forecast correctness and cannot be inflated by copying vocabulary.

## Required behavior

1. Blank or whitespace-only miner answers return exactly `0.0`.
2. Exact ground-truth matches return `1.0`.
3. Equivalent units compare correctly: Celsius/Fahrenheit, km/h–mph–m/s, millimetres/inches.
4. Numeric coverage and accuracy dominate when the ground truth contains weather measurements.
5. Condition families recognize useful paraphrases while penalizing contradictions: clear, cloud, rain, storm, snow, fog, wind, hail, heat, freeze.
6. Temporal scope recognizes today/tomorrow, weekdays, dayparts, and forecast windows.
7. Token stuffing, duplicated answers, and extreme verbosity cannot improve the score.
8. Every result is finite, deterministic, and clamped to `[0,1]`.
9. ABI exports: `memory`, `alloc`, `dealloc`, `rank_answer`, and `breakdown_answer`.

## Scoring model

The scorer chooses weights from the evidence present in the ground truth:

- numeric fidelity and coverage;
- weather-condition agreement and contradiction penalty;
- temporal agreement;
- meaningful lexical overlap after stop-word filtering;
- answer-length quality.

Missing evidence stays missing; it never silently becomes correct. A critical numeric or condition contradiction caps the composite even when surrounding prose is similar.

## Demo

In one terminal command:

1. Run the pinned incumbent and WEATHERPROOF over the same adversarial corpus.
2. Show a correct paraphrase scoring above a wrong answer.
3. Flip only `18°C` to `38°C`, `tomorrow` to `today`, or `clear` to `thunderstorms`.
4. Show WEATHERPROOF's margin and component breakdown.
5. Verify the compiled WASM imports nothing and passes Telegraph's structural contract.

## Honest limitations

- This scorer compares an answer to supplied ground truth; it does not independently fetch or prove weather data.
- It is deterministic domain logic, not a general semantic language model.
- Forecast calibration across many historical events requires aggregation outside the single-call Telegraph ABI.
- Hidden benchmark promotion and historical-ranking agreement can only be proven by registering the final binary.

## Cut order

1. Cut presentation polish.
2. Cut optional corpus cases beyond the minimum adversarial set.
3. Cut multi-intent experiments.

Never cut the ABI, deterministic tests, incumbent comparison, artifact hash, or live registration evidence.

