# ADR 0255: Echo polish, Cap-then-lower multi-word restart phrases collapse

- Status: accepted (echo cycle 139, 2026-11-06)
- Code: crates/qrtz-talk/src/cleanup.rs (`restart_phrase` two-/three-/four-word Cap-then-lower),
  packages/core/src/talk/cleanup.ts (`restartPhrase`), spec/vectors/talk/polish.json (+8 polish
  p329–p336; keep +8 k636–k643; k568/k583 converted to both-lower), polish-baseline.json,
  cleanup.json, tests/hardening139.rs, cleanup.hardening139.test.ts, tests/hardening131.rs
  (Cap-then-lower keep → both-lower), tests/cleanup_budget.rs, tests/hardening60.rs (allowed diff)

## Context

ADR 0246–0250 / 0252 collapse Cap-second multi-word restarts ("as a As a", "Can you Can you",
"How's it going How's it going"). ADR 0254 added Cap-then-lower for one-word contractions
("What's what's"). Fresh Mac prod+dev talk history (2026-11-06 ~3:21 PM PT, 453+443) still left
Cap-then-lower multi-word restarts: "Is that is that one good?" and "You have to you have to…".
ADR 0247 had refused that shape as not Cap-second; Cap-then-lower at sentence start is the same
restart signature ADR 0254 accepted for contractions, now evidenced for 2–4 word phrases.

## Decision

Widen `restart_phrase` two-/three-/four-word matches so a Cap-then-lower form also collapses:
first copy starts uppercase, second copy starts lowercase, last word of each copy lowercase,
`eq_ignore_ascii_case`, spaces/tabs or an ellipsis between copies (`copy_gap_ok`). Cap-second
forms stay. Both-lower multi-word stays (sentence caps may Cap the first word after this step).
Spoken "scratch that", Title-case proper names, and unequal copies stay. Do not add Cap-then-lower
one-word without an apostrophe ("Where where" stays).

No LLM, no network. Rejected this cycle: Cap Cap "So So"; "is Is"/"why Why"/"like Like"; adding
are/is/on/like/why/so/where to STUTTERS; "a An"; lowercase multi-word restarts; "hold on, hold on"
across a comma; content-word doubles; both-lower contraction doubles; email/URL ITN (p19/p20);
p41; unload; speculative N203; N197 mic-switch; year ITN; five-word Cap restart; general
triple-any-word; ", okay"/, yeah/, huh/, eh/, no without unmarked evidence.

## Consequences

"Is that is that one good today" → "Is that one good today?"; "You have to you have to ship it
today" → "You have to ship it today."; "How's it going how's it going for you today" → "How's it
going for you today?"; "Is that... is that one good for us now" → "Is that one good for us now?".
Kept: both-lower "is that is that…", "you have to you have to…"; unequal "Is this is that…";
"New York New York…"; "No, no…"; prior ADR 0254 / 0253 / 0251 behaviour. Polish 325/328 →
**333/336** exact, edit distance 39 (0.32 % of 12036), regressions 0, keep 0/640. Release
`cleanup_budget` Cap-then-lower multi-word median **0.385 ms** (1347-word; ceiling 2 ms).
Capture, segmenting, decode, insert unchanged.
