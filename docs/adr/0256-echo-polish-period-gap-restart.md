# ADR 0256: Echo polish, Cap-then-lower / both-lower restart phrases across a period collapse

- Status: accepted (echo cycle 140, 2026-11-06)
- Code: crates/qrtz-talk/src/cleanup.rs (`restart_phrase` / `copy_gap_ok` / `period_gap`),
  packages/core/src/talk/cleanup.ts (`restartPhrase` / `copyGapOk` / `periodGap`),
  spec/vectors/talk/polish.json (+8 polish p337–p344; keep +8 k644–k651), polish-baseline.json,
  cleanup.json, tests/hardening140.rs, cleanup.hardening140.test.ts, tests/hardening60.rs
  (allowed diff), tests/cleanup_budget.rs

## Context

ADR 0252 allows an ellipsis between Cap-restart copies but refuses a single `.` so Cap Cap across a
period stays (`I will go. I will go`). ADR 0255 Cap-then-lower multi-word still required Cap on the
first copy and a spaces/ellipsis gap. Fresh Mac prod+dev talk history (2026-11-06, 453+443) left
`should be. should be` after cleanup: `restart_phrase` already ran before `continued_dot` dropped
the period, and Cap-then-lower refused both-lower. A single `.` followed by a **lowercase** second
copy is a false segment cut / hesitation, not a sentence boundary — same signature as Cap-then-lower
mid-restart.

## Decision

Widen `restart_phrase` / `restartPhrase`:

1. **`copy_gap_ok`**: allow a single `.` (with surrounding spaces/tabs) iff the second copy starts
   lowercase. Cap Cap across a single period stays refused. Ellipsis and spaces/tabs-only unchanged.
2. **Both-lower multi-word (2/3/4) only when the between-copies gap is a single period** (not
   spaces-only, not ellipsis-only): first may start lower; second starts lower; last word of each
   copy lowercase; `eq_ignore_ascii_case`. Catches `should be. should be` without collapsing
   space-separated keeps like `like a like a`.
3. **Cap-then-lower and Cap-second** also work across the new single-period gap when their Cap
   signatures match (`Thank you. thank you`, `What's. what's`, `Is that. is that`, …).
4. A leftover single period after dropping the second copy is left for `continued_dot` when raw has
   Cap (do not strip single `.` in the drop builder — that broke ellipsis Cap-second parity on
   generated text). Layout commands `new line` / `new paragraph` stay across a period.
5. One-word STUTTERS across a period (`what. what`) is **deferred** this cycle: re-running
   `collapse_repeats` after `continued_dot`, or dropping the second STUTTERS copy inside
   `continued_dot`, hit contraction / newline / filler false positives on the hardening60
   generated corpus. `continued_dot` still drops the period so the double becomes adjacent.

No LLM, no network. Rejected this cycle: Cap Cap across period; space-only both-lower MW; Cap Cap
`So So` / `Where where`; `a An`; adding words to STUTTERS; comma `hold on, hold on`; email/URL ITN;
p41; unload; N203; N197; year ITN; five-word Cap; general triple-any-word; `, okay` without evidence;
one-word STUTTERS period collapse.

## Consequences

`And then the output should be. should be a simple lightweight React animation today` →
`And then the output should be a simple lightweight React animation today.`;
`Thank you. thank you for the review today` → `Thank you for the review today.`;
`Is that. is that one good today` → `Is that one good today?`. Kept: Cap Cap `I will go. I will go…`;
space-only both-lower `like a like a…` / `is that is that…`; `Where where…`; `No. no…` (period dropped
by continued_dot only). Polish 333/336 → **341/344** exact, edit distance 39 (0.32 % of 12325),
regressions 0, keep 0/648. Release `cleanup_budget` period-gap median **0.437 ms** (1227-word;
ceiling 2 ms). Capture, segmenting, decode, insert unchanged.
