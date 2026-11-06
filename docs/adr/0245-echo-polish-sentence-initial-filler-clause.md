# ADR 0245: Echo polish, a sentence-initial filler after a period keeps the sentence mark

- Status: accepted (echo cycle 128, 2026-11-06)
- Code: crates/qrtz-talk/src/cleanup.rs (`opening_filler_clause`, after `opening_filler_marks`),
  packages/core/src/talk/cleanup.ts (`openingFillerClause`), spec/vectors/talk/polish.json (+10 polish p245–p254;
  keep −k496 +14 k543–k556), polish-baseline.json, cleanup.json, tests/hardening128.rs,
  cleanup.hardening128.test.ts, tests/cleanup_budget.rs, tests/hardening83.rs / hardening88.rs (expects)

## Context

ADR 0235 drops a filler said as its own sentence with a mark ("Thanks. Um. Can you send it" → "Thanks. Can you send
it?") but deliberately left "Thanks. Um, can you send it" alone: the comma form was thought to be a segment cut as
often as a sentence. `remove_fillers` still strips that "Um,", leaving "Thanks. can you…"; `continued_dot` (ADR 0120)
then eats the period before the lowercase word, so the insert reads "Thanks can you…".

Prod + dev talk history (453 + 261) now holds many of that shape: "Yeah, it works, thanks. Um, next, can you…",
"thanks. Uh it says loom", "…. Um the first…", "…. Um bad news…", "…. Uh what's…". In every case the period before
the filler was a real sentence end and the glued text is wrong.

## Decision

`opening_filler_clause` runs after `opening_filler_marks`. When a filler sits right after a sentence end (`.!?` +
ASCII blanks), has no mark of its own, and is followed by an optional comma and then either the end of the text or
blanks and a letter:

1. Drop the filler and its comma.
2. Keep the blanks after the sentence mark.
3. Capitalise a lowercase next letter so `continued_dot` cannot drop the mark.

Filler sentences that already carry their own mark stay with ADR 0235. Mid-sentence fillers ("should, uh, ship") and
fillers after a comma ("So, um, next") are unchanged. Kept capital fillers ("UM,") and filler-shaped real words
("Umami", "Uhura", "Umbrella") are refused.

No LLM, no network. Rejected this cycle: widening confirmation tags (", okay" / ", yeah" / ", huh" / ", eh" / ", no")
without an unmarked end-of-sentence hit; email/URL ITN (p19/p20, ADR 0223); p41 (ADR 0188 don't retry); unload;
speculative N203 without a free-relative false "?" in history; N197 (Mac disk ~6 GB free — skipped device-switch).

## Consequences

"Thanks. Um, can you send it" → "Thanks. Can you send it?"; "thanks. Uh it says loom" → "Thanks. It says loom.";
"…. Um, next, …" keeps the period and capitalises "Next". Polish 241/244 → **251/254** exact, edit distance stays 39
(0.42 % of 9293), regressions 0, keep 0/553. Release `cleanup_budget` filler-clause text median **0.216 ms** (ceiling 2 ms). Capture, segmenting,
decode, insert unchanged.
