# ADR 0261: Echo polish, a lowercase "than than" stutter collapses

- Status: accepted (echo cycle 154, 2026-11-06)
- Code: crates/qrtz-talk/src/cleanup.rs (`LOWER_STUTTERS` / `lower_stutter` in `collapses`),
  packages/core/src/talk/cleanup.ts (`LOWER_STUTTERS` / `lowerStutter` in the REPEATS callback),
  spec/vectors/talk/polish.json (+4 polish p377–p380; +6 keep k688–k693),
  polish-baseline.json, cleanup.json (+11), tests/hardening154.rs, cleanup.hardening154.test.ts,
  tests/hardening60.rs (oracle `without_lower_than`), tests/cleanup_budget.rs

## Context

Cached Mac talk history (prod 453 + dev 445, 855 unique raws; Macs offline this cycle) still
types a doubled comparative: "of a symbolic representation than than an actual yeah, if it is an
array". `collapse_repeats` only drops copies of words in STUTTERS, and "than" is not one. Doubled
lowercase "than" with nothing but spaces between the copies has no grammatical reading in
dictation; the only real-world risks are a capitalised copy that is a name ("than Than Shwe"), a
comma restart that may be a clause boundary, or metalinguistic quoting (quotes already break the
repeat match).

## Decision

Add a separate `LOWER_STUTTERS = ["than"]` list (Rust and TS twin). A run of two or more copies
collapses to the first only when **every** byte of the matched run is an ASCII lowercase letter, a
space, or a tab. So:

1. "than than" / "than than than" / "than\tthan" collapse.
2. Any capital ("than Than", "Than than", "than THAN"), any comma, and any line break keep the run.
3. "than" is **not** added to STUTTERS (which is case-insensitive and also feeds COMMA paths).

Across the 855 cached raws the change rewrites exactly one dictation (the one above) and nothing
else. No LLM, no network, no change to capture / segmenting / decode / insert.

Rejected this cycle: "every every", "make make", "edge edge", "multiple multiple" (content-word
doubles, ADR 0254 refuse stands; no new evidence); "it it's"; "hold on, hold on"; "a An"; "So So";
"is Is" / "why Why" / "like Like"; "n no no"; "air air"; lowercase phrase restarts; mid-sentence
"..." Cap continuation (ambiguous new-sentence vs continuation); email/URL ITN (p19/p20); p41;
unload; N203; N197; year ITN.

## Consequences

"It's more of a symbolic representation than than an actual list today" → "It's more of a
symbolic representation than an actual list today."; "It costs more than than than we planned
today" → "It costs more than we planned today.". Kept: "than Than Shwe", "Than than is not a
word", "than THAN", "than, than", "than thanks", "then than". Polish 373/380 → **377/380** exact,
edit distance 64 → **39** chars (0.28 % of 13716), regressions 0, keep 0/689. Release
`cleanup_budget` 495-word median 0.457 ms (0.468 ms before), lowercase-than probe median
**0.817–0.860 ms** (935-word; ceiling 2 ms).
