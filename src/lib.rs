#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(target_arch = "wasm32")]
use core::panic::PanicInfo;

const HEAP_SIZE: usize = 1024 * 1024;
const MAX_MEASUREMENTS: usize = 40;
const MAX_TOKENS: usize = 96;

const KIND_GENERIC: u8 = 0;
const KIND_TEMP: u8 = 1;
const KIND_WIND: u8 = 2;
const KIND_PERCENT: u8 = 3;
const KIND_PRECIP: u8 = 4;

const CLEAR: u16 = 1 << 0;
const CLOUD: u16 = 1 << 1;
const RAIN: u16 = 1 << 2;
const STORM: u16 = 1 << 3;
const SNOW: u16 = 1 << 4;
const FOG: u16 = 1 << 5;
const WIND: u16 = 1 << 6;
const HAIL: u16 = 1 << 7;
const HOT: u16 = 1 << 8;
const COLD: u16 = 1 << 9;
const DRY: u16 = 1 << 10;

static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
static mut HEAP_OFFSET: usize = 0;
static mut BREAKDOWN: [f32; 5] = [0.0; 5];

#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

#[derive(Clone, Copy)]
struct Measurement {
    value: f32,
    kind: u8,
    role: u8,
}

const EMPTY_MEASUREMENT: Measurement = Measurement {
    value: 0.0,
    kind: KIND_GENERIC,
    role: 0,
};

#[derive(Clone, Copy, Debug)]
pub struct ScoreBreakdown {
    pub relevance: f32,
    pub correctness: f32,
    pub lexical: f32,
    pub length_quality: f32,
    pub composite: f32,
}

fn lower(byte: u8) -> u8 {
    if byte.is_ascii_uppercase() {
        byte + 32
    } else {
        byte
    }
}

fn trim_ascii(mut bytes: &[u8]) -> &[u8] {
    while bytes.first().is_some_and(u8::is_ascii_whitespace) {
        bytes = &bytes[1..];
    }
    while bytes.last().is_some_and(u8::is_ascii_whitespace) {
        bytes = &bytes[..bytes.len() - 1];
    }
    bytes
}

fn eq_ci(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| lower(*x) == lower(*y))
}

fn contains_ci(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() || needle.len() > haystack.len() {
        return false;
    }
    haystack
        .windows(needle.len())
        .any(|window| eq_ci(window, needle))
}

fn contains_any(text: &[u8], needles: &[&[u8]]) -> bool {
    needles.iter().any(|needle| contains_ci(text, needle))
}

fn weather_bits(text: &[u8]) -> u16 {
    let mut bits = 0;
    if contains_any(text, &[b"clear", b"sunny", b"fair weather"]) {
        bits |= CLEAR;
    }
    if contains_any(text, &[b"cloud", b"overcast"]) {
        bits |= CLOUD;
    }
    let explicitly_dry = contains_any(
        text,
        &[
            b"no rain",
            b"without rain",
            b"no precipitation",
            b"dry conditions",
            b"remain dry",
        ],
    );
    if explicitly_dry {
        bits |= DRY;
    }
    if !explicitly_dry && contains_any(text, &[b"rain", b"shower", b"drizzle", b"precipitation"]) {
        bits |= RAIN;
    }
    if contains_any(text, &[b"storm", b"thunder", b"lightning", b"cyclone"]) {
        bits |= STORM;
    }
    if contains_any(text, &[b"snow", b"sleet", b"blizzard"]) {
        bits |= SNOW;
    }
    if contains_any(text, &[b"fog", b"mist", b"haze"]) {
        bits |= FOG;
    }
    if contains_any(text, &[b"wind", b"breeze", b"gust", b"gale"]) {
        bits |= WIND;
    }
    if contains_any(text, &[b"hail", b"ice pellet"]) {
        bits |= HAIL;
    }
    if contains_any(text, &[b"hot", b"heatwave", b"warm", b"very warm"]) {
        bits |= HOT;
    }
    if contains_any(
        text,
        &[b"cold", b"freez", b"frost", b"sub-zero", b"subzero"],
    ) {
        bits |= COLD;
    }
    bits
}

fn temporal_bits(text: &[u8]) -> u32 {
    let terms: [&[u8]; 18] = [
        b"today",
        b"tomorrow",
        b"tonight",
        b"morning",
        b"afternoon",
        b"evening",
        b"overnight",
        b"monday",
        b"tuesday",
        b"wednesday",
        b"thursday",
        b"friday",
        b"saturday",
        b"sunday",
        b"weekend",
        b"next week",
        b"this week",
        b"now",
    ];
    let mut bits = 0u32;
    for (index, term) in terms.iter().enumerate() {
        if contains_ci(text, term) {
            bits |= 1 << index;
        }
    }
    bits
}

fn condition_score(expected: u16, answer: u16) -> (f32, bool) {
    if expected == 0 {
        return (0.5, false);
    }
    let matched = (expected & answer).count_ones() as f32;
    let coverage = matched / expected.count_ones() as f32;
    let wet = RAIN | STORM | SNOW | HAIL;
    let contradiction = ((expected & (CLEAR | DRY)) != 0 && (answer & wet) != 0)
        || ((expected & wet) != 0 && (answer & (CLEAR | DRY)) != 0)
        || ((expected & FOG) != 0 && (answer & CLEAR) != 0)
        || ((expected & CLEAR) != 0 && (answer & FOG) != 0)
        || ((expected & SNOW) != 0 && (answer & HOT) != 0)
        || ((expected & HOT) != 0 && (answer & COLD) != 0)
        || ((expected & COLD) != 0 && (answer & HOT) != 0);
    (
        if contradiction {
            coverage * 0.1
        } else {
            coverage
        },
        contradiction,
    )
}

fn temporal_score(expected: u32, answer: u32) -> (f32, bool) {
    if expected == 0 {
        return (0.5, false);
    }
    if answer == 0 {
        return (0.25, false);
    }
    let overlap = (expected & answer).count_ones() as f32;
    let score = overlap / expected.count_ones() as f32;
    const TODAY: u32 = 1 << 0;
    const TOMORROW: u32 = 1 << 1;
    const TONIGHT: u32 = 1 << 2;
    const MORNING: u32 = 1 << 3;
    const AFTERNOON: u32 = 1 << 4;
    const EVENING: u32 = 1 << 5;
    const OVERNIGHT: u32 = 1 << 6;
    let day_conflict = ((expected & TODAY) != 0 && (answer & TOMORROW) != 0)
        || ((expected & TOMORROW) != 0 && (answer & TODAY) != 0);
    let expected_daypart = expected & (TONIGHT | MORNING | AFTERNOON | EVENING | OVERNIGHT);
    let answer_daypart = answer & (TONIGHT | MORNING | AFTERNOON | EVENING | OVERNIGHT);
    let daypart_conflict =
        expected_daypart != 0 && answer_daypart != 0 && (answer_daypart & !expected_daypart) != 0;
    (score, overlap == 0.0 || day_conflict || daypart_conflict)
}

fn is_token_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric()
}

fn is_stopword(token: &[u8]) -> bool {
    const STOPWORDS: [&[u8]; 34] = [
        b"a",
        b"an",
        b"and",
        b"are",
        b"as",
        b"at",
        b"be",
        b"by",
        b"for",
        b"from",
        b"has",
        b"have",
        b"in",
        b"is",
        b"it",
        b"of",
        b"on",
        b"or",
        b"that",
        b"the",
        b"this",
        b"to",
        b"was",
        b"were",
        b"will",
        b"with",
        b"weather",
        b"forecast",
        b"expected",
        b"conditions",
        b"there",
        b"about",
        b"around",
        b"during",
    ];
    STOPWORDS.iter().any(|word| eq_ci(token, word))
}

fn token_hash(token: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in token {
        hash ^= lower(*byte) as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn token_set(text: &[u8]) -> ([u64; MAX_TOKENS], usize, usize) {
    let mut hashes = [0u64; MAX_TOKENS];
    let mut unique = 0usize;
    let mut total = 0usize;
    let mut index = 0usize;
    while index < text.len() {
        while index < text.len() && !is_token_byte(text[index]) {
            index += 1;
        }
        let start = index;
        while index < text.len() && is_token_byte(text[index]) {
            index += 1;
        }
        if start == index {
            continue;
        }
        let token = &text[start..index];
        if token.iter().all(u8::is_ascii_digit) || is_stopword(token) || token.len() == 1 {
            continue;
        }
        total += 1;
        let hash = token_hash(token);
        if !hashes[..unique].contains(&hash) && unique < MAX_TOKENS {
            hashes[unique] = hash;
            unique += 1;
        }
    }
    (hashes, unique, total)
}

fn lexical_f1(expected: &[u8], answer: &[u8]) -> f32 {
    let (expected_hashes, expected_len, _) = token_set(expected);
    let (answer_hashes, answer_len, _) = token_set(answer);
    if expected_len == 0 || answer_len == 0 {
        return 0.0;
    }
    let matches = expected_hashes[..expected_len]
        .iter()
        .filter(|hash| answer_hashes[..answer_len].contains(hash))
        .count() as f32;
    let precision = matches / answer_len as f32;
    let recall = matches / expected_len as f32;
    if precision + recall == 0.0 {
        0.0
    } else {
        2.0 * precision * recall / (precision + recall)
    }
}

fn starts_with_ci(text: &[u8], needle: &[u8]) -> bool {
    text.len() >= needle.len() && eq_ci(&text[..needle.len()], needle)
}

fn unit_kind_and_scale(text: &[u8], start: usize, end: usize) -> (u8, f32, f32) {
    let mut unit_start = end;
    while unit_start < text.len() && matches!(text[unit_start], b' ' | b'\t' | b':' | b'=' | b'\"')
    {
        unit_start += 1;
    }
    let tail = &text[unit_start..core::cmp::min(text.len(), unit_start + 28)];
    if starts_with_ci(tail, b"fahrenheit")
        || starts_with_ci(tail, b"\xc2\xb0f")
        || starts_with_ci(tail, b"deg f")
        || starts_with_ci(tail, b"degrees f")
    {
        return (KIND_TEMP, 5.0 / 9.0, -32.0);
    }
    if starts_with_ci(tail, b"celsius")
        || starts_with_ci(tail, b"\xc2\xb0c")
        || starts_with_ci(tail, b"deg c")
        || starts_with_ci(tail, b"degrees c")
        || matches!(tail, [b'c' | b'C', b' ' | b',' | b'.' | b';' | b'}', ..])
        || tail.eq_ignore_ascii_case(b"c")
    {
        return (KIND_TEMP, 1.0, 0.0);
    }
    if starts_with_ci(tail, b"km/h") || starts_with_ci(tail, b"kmh") || starts_with_ci(tail, b"kph")
    {
        return (KIND_WIND, 1.0, 0.0);
    }
    if starts_with_ci(tail, b"mph") {
        return (KIND_WIND, 1.609_344, 0.0);
    }
    if starts_with_ci(tail, b"m/s")
        || starts_with_ci(tail, b"metres per second")
        || starts_with_ci(tail, b"meters per second")
    {
        return (KIND_WIND, 3.6, 0.0);
    }
    if starts_with_ci(tail, b"percent") || starts_with_ci(tail, b"%") {
        return (KIND_PERCENT, 1.0, 0.0);
    }
    if starts_with_ci(tail, b"inch") || starts_with_ci(tail, b"inches") {
        return (KIND_PRECIP, 25.4, 0.0);
    }
    if starts_with_ci(tail, b"mm") {
        return (KIND_PRECIP, 1.0, 0.0);
    }
    let prefix = &text[start.saturating_sub(24)..start];
    if contains_any(prefix, &[b"temperature", b"high", b"low", b"temp_"]) {
        return (KIND_TEMP, 1.0, 0.0);
    }
    if contains_any(prefix, &[b"wind", b"gust", b"breeze"]) {
        return (KIND_WIND, 1.0, 0.0);
    }
    if contains_any(prefix, &[b"chance", b"probability", b"precip_pct"]) {
        return (KIND_PERCENT, 1.0, 0.0);
    }
    if contains_any(prefix, &[b"rainfall", b"precipitation", b"precip_mm"]) {
        return (KIND_PRECIP, 1.0, 0.0);
    }
    (KIND_GENERIC, 1.0, 0.0)
}

fn nearest_keyword(text: &[u8], needle: &[u8], before: bool) -> usize {
    if needle.len() > text.len() {
        return usize::MAX;
    }
    if before {
        let mut result = usize::MAX;
        for (index, window) in text.windows(needle.len()).enumerate() {
            if eq_ci(window, needle) {
                result = text.len() - index - needle.len();
            }
        }
        result
    } else {
        text.windows(needle.len())
            .position(|window| eq_ci(window, needle))
            .unwrap_or(usize::MAX)
    }
}

fn postfix_keyword_distance(text: &[u8], needle: &[u8]) -> usize {
    let distance = nearest_keyword(text, needle, false);
    if distance == usize::MAX {
        return distance;
    }
    let between = &text[..distance];
    if between.contains(&b',')
        || between.contains(&b';')
        || contains_ci(between, b" and ")
        || contains_ci(between, b" but ")
    {
        usize::MAX
    } else {
        distance
    }
}

fn measurement_role(text: &[u8], start: usize, end: usize) -> u8 {
    let prefix = &text[start.saturating_sub(22)..start];
    let suffix = &text[end..core::cmp::min(text.len(), end + 22)];
    let postfixed_high = [
        b"high".as_slice(),
        b"maximum".as_slice(),
        b"peak".as_slice(),
    ]
    .iter()
    .map(|keyword| postfix_keyword_distance(suffix, keyword))
    .min()
    .unwrap_or(usize::MAX);
    let postfixed_low = [b"low".as_slice(), b"minimum".as_slice()]
        .iter()
        .map(|keyword| postfix_keyword_distance(suffix, keyword))
        .min()
        .unwrap_or(usize::MAX);
    if postfixed_high <= 8 || postfixed_low <= 8 {
        return if postfixed_high < postfixed_low { 1 } else { 2 };
    }
    let groups: [(u8, &[&[u8]]); 4] = [
        (1, &[b"high", b"maximum", b"max_", b"peak"]),
        (2, &[b"low", b"minimum", b"min_"]),
        (3, &[b"chance", b"probability", b"precip_pct"]),
        (4, &[b"threshold", b"exceed", b"above", b"below"]),
    ];
    let mut best_role = 0u8;
    let mut best_distance = usize::MAX;
    for (role, keywords) in groups {
        for keyword in keywords {
            let before_distance = nearest_keyword(prefix, keyword, true);
            if before_distance < best_distance {
                best_distance = before_distance;
                best_role = role;
            }
            // Threshold language usually introduces the following value (for example,
            // "exceeding 50 km/h"). Looking forward would incorrectly attach it to
            // the preceding peak value in "58 km/h, exceeding 50 km/h".
            if role != 4 {
                let after_distance = nearest_keyword(suffix, keyword, false);
                if after_distance < best_distance {
                    best_distance = after_distance;
                    best_role = role;
                }
            }
        }
    }
    best_role
}

fn extract_measurements(text: &[u8]) -> ([Measurement; MAX_MEASUREMENTS], usize) {
    let mut values = [EMPTY_MEASUREMENT; MAX_MEASUREMENTS];
    let mut count = 0usize;
    let mut index = 0usize;
    while index < text.len() && count < MAX_MEASUREMENTS {
        let can_start_negative =
            text[index] == b'-' && index + 1 < text.len() && text[index + 1].is_ascii_digit();
        if !text[index].is_ascii_digit() && !can_start_negative {
            index += 1;
            continue;
        }
        if index > 0 && (text[index - 1].is_ascii_alphabetic() || text[index - 1] == b'#') {
            index += 1;
            continue;
        }
        let start = index;
        if text[index] == b'-' {
            index += 1;
        }
        let mut dot_seen = false;
        while index < text.len() {
            if text[index].is_ascii_digit() || text[index] == b',' {
                index += 1;
            } else if text[index] == b'.' && !dot_seen {
                dot_seen = true;
                index += 1;
            } else {
                break;
            }
        }
        let raw = &text[start..index];
        let mut number = 0.0f32;
        let mut fraction = 0.1f32;
        let mut after_dot = false;
        let mut negative = contains_ci(&text[start.saturating_sub(7)..start], b"minus");
        for byte in raw {
            match *byte {
                b'-' => negative = true,
                b'.' => after_dot = true,
                b',' => {}
                digit if digit.is_ascii_digit() => {
                    if after_dot {
                        number += (digit - b'0') as f32 * fraction;
                        fraction *= 0.1;
                    } else {
                        number = number * 10.0 + (digit - b'0') as f32;
                    }
                }
                _ => {}
            }
        }
        if negative {
            number = -number;
        }
        let (kind, scale, offset) = unit_kind_and_scale(text, start, index);
        let canonical = if kind == KIND_TEMP && offset != 0.0 {
            (number + offset) * scale
        } else {
            number * scale
        };
        values[count] = Measurement {
            value: canonical,
            kind,
            role: measurement_role(text, start, index),
        };
        count += 1;
    }
    (values, count)
}

fn measurement_similarity(expected: Measurement, answer: Measurement) -> f32 {
    if expected.kind != KIND_GENERIC && answer.kind != KIND_GENERIC && expected.kind != answer.kind
    {
        return 0.0;
    }
    if expected.role != 0 && answer.role != 0 && expected.role != answer.role {
        return 0.0;
    }
    let diff = (expected.value - answer.value).abs();
    let tolerance = match expected.kind {
        KIND_TEMP => 5.0,
        KIND_WIND => 18.0,
        KIND_PERCENT => 25.0,
        KIND_PRECIP => expected.value.abs().max(2.0) * 0.5,
        _ => expected.value.abs().max(1.0) * 0.12,
    };
    (1.0 - diff / tolerance).clamp(0.0, 1.0)
}

fn numeric_score(expected: &[u8], answer: &[u8]) -> (f32, usize, usize, bool) {
    let (expected_values, expected_len) = extract_measurements(expected);
    let (answer_values, answer_len) = extract_measurements(answer);
    if expected_len == 0 {
        return (0.5, 0, answer_len, false);
    }
    if answer_len == 0 {
        return (0.0, expected_len, 0, false);
    }
    let mut used = [false; MAX_MEASUREMENTS];
    let mut total = 0.0f32;
    for expected_value in &expected_values[..expected_len] {
        let mut best = 0.0f32;
        let mut best_index = MAX_MEASUREMENTS;
        for (index, answer_value) in answer_values[..answer_len].iter().enumerate() {
            if used[index] {
                continue;
            }
            let candidate = measurement_similarity(*expected_value, *answer_value);
            if candidate > best {
                best = candidate;
                best_index = index;
            }
        }
        if best_index < MAX_MEASUREMENTS {
            used[best_index] = true;
        }
        total += best;
    }
    let coverage_score = total / expected_len as f32;
    let extra = answer_len.saturating_sub(expected_len) as f32;
    let precision_penalty = 1.0 / (1.0 + extra * 0.45);
    let conflicting_extra =
        answer_values[..answer_len]
            .iter()
            .enumerate()
            .any(|(index, answer_value)| {
                !used[index]
                    && expected_values[..expected_len]
                        .iter()
                        .map(|expected_value| {
                            measurement_similarity(*expected_value, *answer_value)
                        })
                        .fold(0.0f32, f32::max)
                        < 0.2
            });
    (
        coverage_score * precision_penalty,
        expected_len,
        answer_len,
        conflicting_extra,
    )
}

fn length_quality(expected: &[u8], answer: &[u8]) -> f32 {
    let expected_len = trim_ascii(expected).len().max(1) as f32;
    let answer_len = trim_ascii(answer).len() as f32;
    let ratio = answer_len / expected_len;
    let mut quality = if ratio < 0.15 {
        ratio / 0.15
    } else if ratio <= 2.5 {
        1.0
    } else if ratio <= 5.0 {
        1.0 - (ratio - 2.5) * 0.2
    } else {
        0.3
    };
    let (_, unique, total) = token_set(answer);
    if total > 24 {
        let diversity = unique as f32 / total as f32;
        if diversity < 0.35 {
            quality *= (diversity / 0.35).max(0.2);
        }
    }
    quality.clamp(0.0, 1.0)
}

pub fn score_bytes(question: &[u8], ground_truth: &[u8], miner_answer: &[u8]) -> ScoreBreakdown {
    let expected = trim_ascii(ground_truth);
    let answer = trim_ascii(miner_answer);
    if answer.is_empty() {
        return ScoreBreakdown {
            relevance: 0.0,
            correctness: 0.0,
            lexical: 0.0,
            length_quality: 0.0,
            composite: 0.0,
        };
    }
    if eq_ci(expected, answer) {
        return ScoreBreakdown {
            relevance: 1.0,
            correctness: 1.0,
            lexical: 1.0,
            length_quality: 1.0,
            composite: 1.0,
        };
    }

    let lexical = lexical_f1(expected, answer);
    let relevance = if trim_ascii(question).is_empty() {
        0.5
    } else {
        lexical_f1(question, answer)
    };
    let (numbers, expected_number_count, answer_number_count, conflicting_extra_number) =
        numeric_score(expected, answer);
    let (conditions, condition_contradiction) =
        condition_score(weather_bits(expected), weather_bits(answer));
    let (time, time_contradiction) = temporal_score(temporal_bits(expected), temporal_bits(answer));
    let length = length_quality(expected, answer);

    let correctness = if expected_number_count > 0 {
        0.62 * numbers + 0.25 * conditions + 0.13 * time
    } else if weather_bits(expected) != 0 {
        0.68 * conditions + 0.32 * time
    } else {
        0.55 * lexical + 0.25 * time + 0.20 * relevance
    };

    let mut composite = if expected_number_count > 0 {
        0.58 * numbers + 0.20 * conditions + 0.09 * time + 0.10 * lexical + 0.03 * relevance
    } else if weather_bits(expected) != 0 {
        0.55 * conditions + 0.12 * time + 0.28 * lexical + 0.05 * relevance
    } else {
        0.72 * lexical + 0.16 * time + 0.12 * relevance
    };

    if condition_contradiction {
        composite *= 0.12;
    }
    if conflicting_extra_number {
        composite *= 0.15;
    }
    if time_contradiction {
        composite *= 0.45;
    }
    if expected_number_count > 0 && answer_number_count > 0 && numbers < 0.35 {
        composite *= 0.12;
    } else if expected_number_count > 0 && answer_number_count == 0 {
        composite *= 0.5;
    }
    composite *= length;
    composite = contrast(contrast(composite));

    ScoreBreakdown {
        relevance: relevance.clamp(0.0, 1.0),
        correctness: correctness.clamp(0.0, 1.0),
        lexical: lexical.clamp(0.0, 1.0),
        length_quality: length,
        composite: composite.clamp(0.0, 1.0),
    }
}

fn contrast(value: f32) -> f32 {
    let clamped = value.clamp(0.0, 1.0);
    let positive = clamped * clamped;
    let negative = (1.0 - clamped) * (1.0 - clamped);
    if positive + negative == 0.0 {
        0.0
    } else {
        positive / (positive + negative)
    }
}

#[no_mangle]
pub unsafe extern "C" fn alloc(size: i32) -> i32 {
    let size = size.max(0) as usize;
    if size > HEAP_SIZE {
        return 0;
    }
    let mut aligned = (HEAP_OFFSET + 7) & !7;
    if aligned + size > HEAP_SIZE {
        aligned = 0;
    }
    HEAP_OFFSET = aligned + size;
    core::ptr::addr_of_mut!(HEAP).cast::<u8>().add(aligned) as i32
}

#[no_mangle]
pub unsafe extern "C" fn dealloc(_ptr: i32, _size: i32) {}

unsafe fn read_bytes<'a>(ptr: i32, len: i32) -> &'a [u8] {
    if ptr <= 0 || len <= 0 {
        return &[];
    }
    core::slice::from_raw_parts(ptr as *const u8, len as usize)
}

unsafe fn score_from_pointers(
    q_ptr: i32,
    q_len: i32,
    gt_ptr: i32,
    gt_len: i32,
    ma_ptr: i32,
    ma_len: i32,
) -> ScoreBreakdown {
    score_bytes(
        read_bytes(q_ptr, q_len),
        read_bytes(gt_ptr, gt_len),
        read_bytes(ma_ptr, ma_len),
    )
}

#[no_mangle]
pub unsafe extern "C" fn rank_answer(
    q_ptr: i32,
    q_len: i32,
    gt_ptr: i32,
    gt_len: i32,
    ma_ptr: i32,
    ma_len: i32,
) -> f32 {
    score_from_pointers(q_ptr, q_len, gt_ptr, gt_len, ma_ptr, ma_len).composite
}

#[no_mangle]
pub unsafe extern "C" fn breakdown_answer(
    q_ptr: i32,
    q_len: i32,
    gt_ptr: i32,
    gt_len: i32,
    ma_ptr: i32,
    ma_len: i32,
) -> i32 {
    let score = score_from_pointers(q_ptr, q_len, gt_ptr, gt_len, ma_ptr, ma_len);
    BREAKDOWN = [
        score.relevance,
        score.correctness,
        score.lexical,
        score.length_quality,
        score.composite,
    ];
    core::ptr::addr_of!(BREAKDOWN) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn score(expected: &str, answer: &str) -> f32 {
        score_bytes(b"weather question", expected.as_bytes(), answer.as_bytes()).composite
    }

    #[test]
    fn blank_is_exact_zero() {
        assert_eq!(score("Sunny, 20 C", "   \n\t"), 0.0);
    }

    #[test]
    fn exact_match_is_one() {
        assert_eq!(score("Rain tomorrow, 18 C", "rain TOMORROW, 18 c"), 1.0);
    }

    #[test]
    fn converts_temperature_units() {
        let equivalent = score(
            "Tomorrow will reach 20 Celsius",
            "Tomorrow will reach 68 Fahrenheit",
        );
        let wrong = score(
            "Tomorrow will reach 20 Celsius",
            "Tomorrow will reach 100 Fahrenheit",
        );
        assert!(equivalent > wrong + 0.5, "{equivalent} vs {wrong}");
    }

    #[test]
    fn converts_wind_units() {
        let equivalent = score("Wind gusts reach 36 km/h", "Wind gusts reach 10 m/s");
        let wrong = score("Wind gusts reach 36 km/h", "Wind gusts reach 30 m/s");
        assert!(equivalent > wrong + 0.45, "{equivalent} vs {wrong}");
    }

    #[test]
    fn condition_contradiction_collapses_score() {
        let good = score(
            "Tomorrow stays clear and dry",
            "Sunny tomorrow with no rain",
        );
        let bad = score(
            "Tomorrow stays clear and dry",
            "Thunderstorms and heavy rain tomorrow",
        );
        assert!(good > bad + 0.5, "{good} vs {bad}");
    }

    #[test]
    fn time_window_flip_is_penalized() {
        let good = score(
            "Rain tomorrow morning, 70 percent",
            "Tomorrow morning has a 70% rain chance",
        );
        let bad = score(
            "Rain tomorrow morning, 70 percent",
            "Today evening has a 70% rain chance",
        );
        assert!(good > bad + 0.25, "{good} vs {bad}");
    }

    #[test]
    fn token_stuffing_cannot_help() {
        let expected = "Tomorrow: rain, high 18 Celsius, wind 20 km/h";
        let good = score(expected, "Rain tomorrow; high 18 C and wind 20 km/h");
        let stuffed = score(expected, &"rain weather forecast tomorrow ".repeat(80));
        assert!(good > stuffed + 0.4, "{good} vs {stuffed}");
    }

    #[test]
    fn supports_postfixed_high_and_low_labels() {
        let expected = "Cloudy tomorrow, high 23 Celsius, low 14 Celsius, rain chance 65 percent";
        let equivalent = "Cloudy tomorrow: 23°C high, 14°C low, chance of rain 65%";
        let value = score(expected, equivalent);
        assert!(value > 0.9, "{value}");
    }

    #[test]
    fn keeps_roles_with_connectors_between_values() {
        let expected = "Lagos tomorrow will be partly cloudy with a high of 31 Celsius and a low of 25 Celsius";
        let equivalent = "Tomorrow in Lagos: partly cloudy, high 31°C and low 25°C";
        let value = score(expected, equivalent);
        assert!(value > 0.9, "{value}");
    }

    #[test]
    fn scores_peak_and_threshold_roles() {
        let expected = "Tokyo gusts will peak at 58 km/h tomorrow, above the 50 km/h threshold";
        let equivalent = "Tomorrow's peak gust in Tokyo is 58 km/h, exceeding 50 km/h";
        let breakdown = score_bytes(
            b"weather question",
            expected.as_bytes(),
            equivalent.as_bytes(),
        );
        assert!(breakdown.composite > 0.95, "{breakdown:?}");
    }

    #[test]
    fn score_is_bounded() {
        for answer in ["nonsense", "🌧️ 東京 20°C", "-99999 mph", "{}"] {
            let value = score("Rain tomorrow at 20°C with 40% probability", answer);
            assert!(value.is_finite() && (0.0..=1.0).contains(&value));
        }
    }
}
