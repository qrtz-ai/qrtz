# ADR 0248: Echo polish, one-word Cap contraction restart collapses

- Status: accepted (echo cycle 132, 2026-11-06)
- Code: crates/qrtz-talk/src/cleanup.rs (`restart_phrase`, after `collapse_repeats`),
  packages/core/src/talk/cleanup.ts (`restartPhrase`), spec/vectors/talk/polish.json (+8 polish
  p273–p280; keep +10 k574–k583), polish-baseline.json, cleanup.json, tests/hardening132.rs,
  cleanup.hardening132.test.ts, tests/cleanup_budget.rs, tests/hardening60.rs (oracle)

## Context

ADR 0246 / ADR 0247 collapse two- and three-word Cap restarts ("as a As a", "Can you Can you",
"I don't know I don't know"). Fresh Mac prod+dev talk history (453 + 306) still shows a one-word
Cap restart of a contraction: "it's It's not correct". `collapse_repeats` / STUTTERS only see
ASCII `\w` runs, so an apostrophe splits "it's" and the Cap copy never collapses. Plain Cap
stutters without an apostrophe stay on the STUTTERS allowlist ("it It", "the The") or are refused
("like Like", "will Will", "so So") — those are intentional discourse or name shapes.

## Decision

Widen `restart_phrase` (still after `collapse_repeats`) with a one-word Cap form: the word
contains an ASCII or curly apostrophe; the first copy starts lowercase (mid-sentence); the second
starts uppercase; `eq_ignore_ascii_case`; only spaces/tabs between. Prefer three- then two- then
one-word matches. Both-Cap "It's It's" / "O'Brien O'Brien" stay. Same-case "it's it's" /
"don't don't" stay. No LLM, no network.

Rejected this cycle: ", okay"/", yeah"/", huh"/", eh"/", no" wideners (still 0 unmarked end hits);
"like Like" / adding "like"/"so" to STUTTERS; lowercase "like a like a" phrase stutter (ADR 0247
refuse of lowercase-second-copy without Cap); email/URL ITN (p19/p20, ADR 0223); p41 (ADR 0188
don't retry); unload; speculative N203; N197 (needs a quiet mic-switch window; Mac disk ~16 GB
free but still disruptive); year ITN (k350).

## Consequences

"it's It's not correct" → "it's not correct"; "don't Don't want…" → "don't want…";
"can't Can't really…" → "can't really…". Kept: "it's it's", "It's It's", "will Will",
"like Like", "is Is", "O'Brien O'Brien", "like a like a", prior two-/three-word keeps.
Polish 269/272 → **277/280** exact, edit distance stays 39 (0.38 % of 10301), regressions 0, keep 0/580.
Release `cleanup_budget` Cap-contraction probe median **0.228 ms**. Capture, segmenting, decode,
insert unchanged.
