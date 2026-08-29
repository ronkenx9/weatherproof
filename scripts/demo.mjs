#!/usr/bin/env node

import { readFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import path from "node:path";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const bytes = await readFile(
  path.join(root, "target/wasm32-unknown-unknown/release/weatherproof_scorer.wasm"),
);
const instance = await WebAssembly.instantiate(bytes, {});
const { memory, alloc, rank_answer: rank, breakdown_answer: breakdown } = instance.instance.exports;
const encoder = new TextEncoder();

function put(text) {
  const encoded = encoder.encode(text);
  const pointer = alloc(encoded.length);
  new Uint8Array(memory.buffer, pointer, encoded.length).set(encoded);
  return [pointer, encoded.length];
}

function run(label, answer) {
  const question = put("Will Lagos be safe for an outdoor event tomorrow morning?");
  const truth = put("Lagos tomorrow morning will stay clear and dry at 31 Celsius with wind near 18 km/h.");
  const candidate = put(answer);
  const args = [...question, ...truth, ...candidate];
  const score = rank(...args);
  const pointer = breakdown(...args);
  const [relevance, correctness, lexical, lengthQuality, composite] = new Float32Array(
    memory.buffer,
    pointer,
    5,
  );
  console.log(
    `${label.padEnd(12)} score=${score.toFixed(3)}  facts=${correctness.toFixed(3)}  ` +
      `words=${lexical.toFixed(3)}  length=${lengthQuality.toFixed(3)}  final=${composite.toFixed(3)}`,
  );
}

console.log("WEATHERPROOF — score the forecast facts, not the prose\n");
run("correct", "Clear and dry tomorrow morning in Lagos: 31°C, winds around 18 km/h.");
run("temp flip", "Clear and dry tomorrow morning in Lagos: 51°C, winds around 18 km/h.");
run("storm flip", "Heavy rain and thunderstorms tomorrow morning in Lagos: 31°C, winds around 18 km/h.");
run("word stuffing", "Lagos tomorrow morning clear dry weather forecast temperature wind conditions outdoor event. Actual: 51°C with thunderstorms.");

