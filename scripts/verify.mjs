#!/usr/bin/env node

import { readFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import path from "node:path";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const wasmPath = path.join(
  root,
  "target/wasm32-unknown-unknown/release/weatherproof_scorer.wasm",
);
const corpusPath = path.join(root, "bench/weather-forecast.json");

const [candidateBytes, corpus] = await Promise.all([
  readFile(wasmPath),
  readFile(corpusPath, "utf8").then(JSON.parse),
]);
const incumbentResponse = await fetch(corpus.incumbent.url);
if (!incumbentResponse.ok) {
  throw new Error(`incumbent fetch failed: HTTP ${incumbentResponse.status}`);
}
const incumbentBytes = new Uint8Array(await incumbentResponse.arrayBuffer());

function createRunner(bytes, requireBreakdown = false) {
  const module = new WebAssembly.Module(bytes);
  const imports = WebAssembly.Module.imports(module);
  if (imports.length !== 0) {
    throw new Error(`WASM imports are forbidden: ${JSON.stringify(imports)}`);
  }
  const instance = new WebAssembly.Instance(module, {});
  const { memory, alloc, dealloc, rank_answer: rankAnswer, breakdown_answer: breakdownAnswer } =
    instance.exports;
  for (const [name, value] of Object.entries({ memory, alloc, dealloc, rankAnswer })) {
    if (!value) throw new Error(`missing required export: ${name}`);
  }
  if (requireBreakdown && !breakdownAnswer) {
    throw new Error("missing required practical export: breakdown_answer");
  }
  const encoder = new TextEncoder();

  function put(text) {
    const bytes = encoder.encode(text);
    const pointer = alloc(bytes.length);
    if (!pointer && bytes.length) throw new Error("allocator returned null");
    new Uint8Array(memory.buffer, pointer, bytes.length).set(bytes);
    return { pointer, length: bytes.length };
  }

  function score(question, groundTruth, answer) {
    const q = put(question);
    const gt = put(groundTruth);
    const ma = put(answer);
    const value = rankAnswer(q.pointer, q.length, gt.pointer, gt.length, ma.pointer, ma.length);
    dealloc(q.pointer, q.length);
    dealloc(gt.pointer, gt.length);
    dealloc(ma.pointer, ma.length);
    if (!Number.isFinite(value) || value < 0 || value > 1) {
      throw new Error(`invalid score: ${value}`);
    }
    return value;
  }

  function breakdown(question, groundTruth, answer) {
    if (!breakdownAnswer) return null;
    const q = put(question);
    const gt = put(groundTruth);
    const ma = put(answer);
    const pointer = breakdownAnswer(
      q.pointer,
      q.length,
      gt.pointer,
      gt.length,
      ma.pointer,
      ma.length,
    );
    const values = [...new Float32Array(memory.buffer, pointer, 5)];
    dealloc(q.pointer, q.length);
    dealloc(gt.pointer, gt.length);
    dealloc(ma.pointer, ma.length);
    return Object.fromEntries(
      ["relevance", "correctness", "lexical", "length_quality", "composite"].map(
        (key, index) => [key, values[index]],
      ),
    );
  }

  return { imports, exports: Object.keys(instance.exports), score, breakdown };
}

const candidate = createRunner(candidateBytes, true);
const incumbent = createRunner(incumbentBytes);

function evaluate(runner) {
  let wins = 0;
  let attackWins = 0;
  let margin = 0;
  let attackMargin = 0;
  const rows = [];
  for (const testCase of corpus.cases) {
    const good = runner.score(testCase.question, testCase.ground_truth, testCase.good);
    const bad = runner.score(testCase.question, testCase.ground_truth, testCase.bad);
    const attack = runner.score(testCase.question, testCase.ground_truth, testCase.attack);
    if (good > bad) wins += 1;
    if (good > attack) attackWins += 1;
    margin += good - bad;
    attackMargin += good - attack;
    rows.push({ id: testCase.id, good, bad, attack, margin: good - bad });
  }
  return {
    wins,
    attackWins,
    cases: corpus.cases.length,
    averageMargin: margin / corpus.cases.length,
    averageAttackMargin: attackMargin / corpus.cases.length,
    rows,
  };
}

const candidateResult = evaluate(candidate);
const incumbentResult = evaluate(incumbent);

const blank = candidate.score("weather", "Sunny", " \n\t ");
const selfScores = corpus.cases.map((testCase) =>
  candidate.score(testCase.question, testCase.ground_truth, testCase.ground_truth),
);
const worstSelf = Math.min(...selfScores);
if (blank !== 0) throw new Error(`blank answer must be 0, got ${blank}`);
if (worstSelf < 0.75) throw new Error(`worst self-match ${worstSelf} is below 0.75`);

const unicode = candidate.score(
  "東京の明日の天気は？ 🌧️",
  "東京は明日、雨で20 Celsiusです。",
  "東京は明日、雨で20°Cです。",
);
const longText = "weather 🌦️ 東京 20 Celsius ".repeat(4_500);
candidate.score("long input", longText, longText);

let seed = 42;
for (let index = 0; index < 500; index += 1) {
  seed = (seed * 1_664_525 + 1_013_904_223) >>> 0;
  const answer = `${seed % 80} Celsius ${seed % 2 ? "rain" : "clear"} ${"x".repeat(seed % 97)}`;
  const first = candidate.score("fuzz weather", "Rain tomorrow at 20 Celsius", answer);
  const second = candidate.score("fuzz weather", "Rain tomorrow at 20 Celsius", answer);
  if (first !== second) throw new Error(`non-deterministic fuzz case ${index}`);
}

const regressions = candidateResult.rows.filter((row, index) => {
  const baseline = incumbentResult.rows[index];
  return row.margin < baseline.margin;
});
const passed =
  candidateResult.wins === candidateResult.cases &&
  candidateResult.attackWins === candidateResult.cases &&
  candidateResult.averageMargin > incumbentResult.averageMargin &&
  regressions.length === 0;

console.log(
  JSON.stringify(
    {
      intent: corpus.intent,
      candidate: {
        ...candidateResult,
        bytes: candidateBytes.length,
        imports: candidate.imports,
        exports: candidate.exports,
        blank,
        worstSelf,
        unicode,
      },
      incumbent: {
        registrationId: corpus.incumbent.registration_id,
        recordedLiveMargin: corpus.incumbent.margin,
        ...incumbentResult,
      },
      regressions: regressions.map((row) => row.id),
      passed,
    },
    null,
    2,
  ),
);

if (!passed) process.exitCode = 1;

