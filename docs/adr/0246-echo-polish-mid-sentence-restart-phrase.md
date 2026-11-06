# ADR 0246: Echo polish, a mid-sentence two-word restart phrase collapses to one copy

- Status: accepted (echo cycle 130, 2026-11-06)
- Code: crates/qrtz-talk/src/cleanup.rs (`restart_phrase`, after `collapse_repeats`),
  packages/core/src/talk/cleanup.ts (`restartPhrase`), spec/vectors/talk/polish.json (+8 polish p255–p262;
  keep +10 k557–k566), polish-baseline.json, cleanup.json, tests/hardening130.rs,
  cleanup.hardening130.test.ts, tests/cleanup_budget.rs, tests/hardening60.rs (oracle)

## Context

Parakeet sometimes restarts a short phrase mid-sentence with a capital on the first word and no sentence mark
between the copies: "just use it as a As a mock-up", "in a In a way", "in the In the best interest", "it doesn't
It doesn't quite". Prod talk history was quiet on this shape; today's fresh Mac **dev** talk history added
"as a As a", and the same pattern already sat in older prod/dev entries. Single-word STUTTERS already collapse
"and And" / "you You"; these two-word restarts were left as typed.

A Title-case **both** words form ("new york New York", "app store App Store") is a proper name, not a restart.
"scratch that Scratch that" is ADR 0186's spoken command, not a stutter. A newline between copies is a real
sentence start.

## Decision

`restart_phrase` runs after `collapse_repeats`. When four adjacent words A B A' B' have only spaces/tabs between
them, A and B are all-lowercase, A' starts uppercase, B' starts lowercase, A≈A' and B≈B' case-insensitively, and
the phrase is not "scratch that", drop A' B' (and the blanks before them).

No LLM, no network. Rejected this cycle: ", okay"/", yeah"/", huh"/", eh"/", no" wideners (still 0 unmarked end
hits); period-marked ", right."; inverted tags; "which"/"should" question openers; email/URL ITN (p19/p20, ADR
0223); p41 (ADR 0188 don't retry); unload; speculative N203 without a free-relative false "?" in history; N197
(Mac disk ~1.5 GB free — skipped device-switch); year ITN (k350); adding content verbs ("make") to STUTTERS.

## Consequences

"just use it as a As a mock-up" → "Just use it as a mock-up."; "in a In a way" → "in a way"; "it doesn't It
doesn't quite" → "it doesn't quite". Kept: "new york New York", "app store App Store", "scratch that\nScratch
that", space-separated "scratch that Scratch that". Polish 251/254 → **259/262** exact, edit distance stays 39
(0.40 % of 9682), regressions 0, keep 0/563. Release `cleanup_budget` restart-phrase text median **0.208 ms**
(ceiling 2 ms). Capture, segmenting, decode, insert unchanged.
