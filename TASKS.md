# WEATHERPROOF — Execution Tasks

The first unchecked task is the only active task. Every checked task must have its gate rerun.

## P0 — Scout and select

- [x] P0.1 Refresh live miner catalog and count intent coverage.
  - Gate: node reports 106 miners and 14 `WEATHER_FORECAST` declarations.
- [x] P0.2 Enumerate Base Sepolia WASM registry and current champions.
  - Gate: 1,784 active scripts, 44 distinct authors, champion #636 margin `0.53020585` recorded.
- [x] P0.3 Run ideation gates and reject alternatives.
  - Gate: verdict and also-rans persisted in the Brain Idea Bank.

## P1 — Core scorer

- [x] P1.1 Implement bounded parser and evidence extractors.
  - Gate: native unit tests cover numbers, units, conditions, time, blank and exact match.
- [x] P1.2 Implement composite and anti-gaming caps.
  - Gate: every adversarial better answer ranks above its paired corrupted/stuffed answer.
- [x] P1.3 Export Telegraph ABI including breakdown.
  - Gate: WebAssembly inspection finds zero imports and all required exports.

## P2 — Evaluation

- [x] P2.1 Build the pinned 15-case weather corpus.
  - Gate: corpus covers value, unit, condition, location, time-window, omission, contradiction, and stuffing attacks.
- [x] P2.2 Compare candidate against pinned incumbent #636.
  - Gate: candidate wins every local pair and beats incumbent on pairwise wins and average separation without regressions.
- [x] P2.3 Fuzz and stress the artifact.
  - Gate: deterministic finite `[0,1]` outputs; blank exactly zero; 128 KiB and Unicode do not trap.

## P3 — Package

- [x] P3.1 Generate registration manifest.
  - Gate: intent, keccak256, byte size, source commit, incumbent evidence, and public artifact URL are present.
- [x] P3.2 Fresh-clone verification and CI.
  - Gate: README commands pass from a clean checkout.
- [x] P3.3 Prepare submission and X drafts.
  - Gate: no claims exceed recorded evidence; owner-only publish actions are clearly marked.

## P4 — Live registration

- [x] P4.1 Publish repository and immutable artifact URL.
  - Gate: URL returns exact bytes matching manifest keccak256.
- [x] P4.2 Register from owner-controlled Base Sepolia wallet.
  - Gate: `WasmRegistered` event and registration ID captured.
- [ ] P4.3 Verify Telegraph evaluation.
  - Gate: status, candidate/champion margins, wins, and historical agreement captured. Fix and re-register if rejected while time remains.
