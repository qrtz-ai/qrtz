# ADR 0247: Echo polish, Cap-leading two-word and three-word restart phrases collapse

- Status: accepted (echo cycle 131, 2026-11-06)
- Code: crates/qrtz-talk/src/cleanup.rs (`restart_phrase`, after `collapse_repeats`),
  packages/core/src/talk/cleanup.ts (`restartPhrase`), spec/vectors/talk/polish.json (+10 polish
  p263–p272; keep +7 k567–k573), polish-baseline.json, cleanup.json, tests/hardening131.rs,
  cleanup.hardening131.test.ts, tests/cleanup_budget.rs, tests/hardening60.rs (oracle)

## Context

ADR 0246 collapsed a mid-sentence two-word restart only when the first copy was all-lowercase
("as a As a"). Fresh Mac prod+dev talk history (453 + 270) still shows Cap-leading two-word restarts
("Can you Can you", "I don't I don't", "Let's just Let's just", "I guess I guess", "It would It
would", "That is That is") and three-word Cap restarts ("I don't know I don't know", "How's it going
How's it going", "What are the What are the", "how I want How I want"). Those stayed typed.

Title-case proper names ("new york New York", "New York City New York City") stay out because the
last word of the second copy starts uppercase. Spoken "scratch that" stays a command. A second copy
that starts lowercase ("You have to you have to", "the starting deck the starting deck") is not the
Parakeet Cap-restart shape and is left alone.

## Decision

Widen `restart_phrase` (still after `collapse_repeats`):

1. Two-word: drop the all-lowercase requirement on the first word of the first copy so Title-case
   "Can you Can you" collapses. Keep requiring the last word of each copy to be lowercase.
2. Three-word: the same Cap-restart shape on six adjacent words (spaces/tabs only). A middle word
   may be the pronoun "I" / "I'd" / "I'm" / "I've" / "I'll". Prefer the longer match when both
   could apply.

No LLM, no network. Rejected this cycle: ", okay"/", yeah"/", huh"/", eh"/", no" wideners (still 0
unmarked end hits); period-marked ", right."; inverted tags; "which"/"should" question openers;
email/URL ITN (p19/p20, ADR 0223); p41 (ADR 0188 don't retry); unload; speculative N203 without a
free-relative false "?" in history; N197 (Mac disk ~1.3 GB free — skipped); year ITN (k350); adding
"so"/"like" to STUTTERS ("so so" mediocre; "like like" discourse); collapsing lowercase-second-copy
phrase stutters ("You have to you have to") without Cap evidence.

## Consequences

"Can you Can you host…" → "Can you host…?"; "I don't I don't like…" → "I don't like…"; "I don't
know I don't know how…" → "I don't know how…"; "What are the What are the deciding…" → "What are
the deciding…?". Kept: "new york New York", "New York City New York City", "You have to you have
to", "the starting deck the starting deck", "scratch that Scratch that". Polish 259/262 →
**269/272** exact, edit distance stays 39 (0.39 % of 10047), regressions 0, keep 0/570. Release
`cleanup_budget` restart-phrase text median **0.246 ms** (ceiling 2 ms; longer probe than cycle
130). Capture, segmenting, decode, insert unchanged.
