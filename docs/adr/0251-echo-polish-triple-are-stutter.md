# ADR 0251: Echo polish, triple+ "are" stutter collapses

- Status: accepted (echo cycle 135, 2026-11-06)
- Code: crates/qrtz-talk/src/cleanup.rs (`collapses`, after STUTTERS / COMMA_STUTTERS),
  packages/core/src/talk/cleanup.ts (REPEATS callback), spec/vectors/talk/polish.json (+8 polish
  p297–p304; keep +8 k604–k611), polish-baseline.json, cleanup.json, tests/hardening135.rs,
  cleanup.hardening135.test.ts, tests/cleanup_budget.rs, tests/hardening60.rs (oracle)

## Context

STUTTERS collapses immediate doubles of stutter-prone words ("the the", "I I"). "are" and "is" stay
out of that list because grammatical / restart doubles are real English: "you are Are you sure",
"is is that correct", "What are Are we doing". Fresh Mac prod+dev talk history (2026-11-06 ~12:19
PT) still typed a triple: "Are are are we always fetching on hover?" — three consecutive "are" is
never intentional, and the double-refusal reason does not apply.

## Decision

Extend `collapses` (and the TS REPEATS callback) so that after the STUTTERS / COMMA_STUTTERS check,
a run of **three or more** space/tab-separated "are" (any ASCII case via eq_ignore_ascii_case /
toLowerCase) collapses to the first copy. Doubles stay. Never collapse across commas. Do **not**
add "are" to STUTTERS.

No LLM, no network. Rejected this cycle: adding "are" to STUTTERS (doubles — "you are Are you");
adding "on"/"is"/"why"/"like" to STUTTERS; ", okay"/", yeah"/", huh"/", eh"/", no" wideners;
email/URL ITN (p19/p20, ADR 0223); p41 (ADR 0188 don't retry); unload; speculative N203; N197
mic-switch; year ITN (k350); five-word Cap restart (0 hits); general triple-any-word collapse
(breaks "No no no" / "Wait wait wait" emphasis).

## Consequences

"Are are are we always fetching on hover" → "Are we always fetching on hover?"; "we are are are
going tomorrow" → "We are going tomorrow."; "Are are are are we done yet today" → "Are we done yet
today?". Kept: "you are Are you sure about that", "The results are are interesting today", "Are Are
we ready to ship", "is is that correct though", "turn it on on Friday please", prior ADR
0248/0250 keeps. Polish 293/296 → **301/304** exact, edit distance stays 39, regressions 0, keep
0/608. Release `cleanup_budget` triple-are probe median **0.247 ms** (865-word; ceiling 2 ms). Capture, segmenting,
decode, insert unchanged.
