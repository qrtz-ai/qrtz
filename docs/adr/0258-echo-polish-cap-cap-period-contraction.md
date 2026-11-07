# ADR 0258: Echo polish, Cap Cap one-word verbal contraction across a period collapses

- Status: accepted (echo cycle 142, 2026-11-06)
- Code: crates/qrtz-talk/src/cleanup.rs (`restart_phrase` / `period_gap` / `is_verbal_contraction`),
  packages/core/src/talk/cleanup.ts (`restartPhrase` / `periodGap` / `isVerbalContraction`),
  spec/vectors/talk/polish.json (+8 polish p353–p360; keep +10 k660–k669), polish-baseline.json,
  cleanup.json, tests/hardening142.rs, cleanup.hardening142.test.ts, tests/hardening60.rs
  (allowed diff), tests/cleanup_budget.rs

## Context

ADR 0256 allows a single `.` between restart copies only when the second copy starts lowercase, so
Cap Cap across a period stays refused (`I will go. I will go`). ADR 0254 collapses Cap-then-lower
one-word contractions (`I'm. i'm`) but not Cap Cap (`I'm. I'm`). Fresh Mac prod+dev talk history
(2026-11-06, 453+443) left a real restart: `Like, I'm. I'm a senior engineer now` after cleanup —
Parakeet wrote a false segment cut between two Cap copies of the same verbal contraction.

## Decision

Widen `restart_phrase` / `restartPhrase` one-word form:

1. **Cap Cap across a single period** collapses when both copies start uppercase, `eq_ignore_ascii_case`,
   the gap is exactly spaces/tabs around one `.` (not spaces-only, not ellipsis), and **both** words
   are verbal / aux contractions (`is_verbal_contraction`): suffix `n't` / `'m` / `'re` / `'ve` /
   `'ll` / `'d` / `'t`, or `'s` with stem in
   `{it,that,what,who,where,when,why,how,there,here,he,she,let}`.
2. Cap Cap across spaces/ellipsis stays refused (`I'm I'm`, `I'm... I'm`, `It's It's`).
3. Cap Cap without a verbal contraction stays refused (`So. So`, `Yes. Yes`, `I. I`, `Where. Where`,
   multi-word `I will go. I will go`).
4. Names and arbitrary possessives stay refused (`O'Brien. O'Brien`, `Bob's. Bob's`) — the verbal
   filter excludes them.
5. Dropping the second Cap copy also strips the single period in the drop builder (so the result
   does not depend on `continued_dot` seeing a lowercase continuation).

No LLM, no network. Rejected this cycle: Cap Cap non-contraction across period; Cap Cap contraction
across spaces/ellipsis; Cap Cap `So So` / `Where where`; `a An`; adding are/is/on/like/why/so/where
to STUTTERS; `hold on, hold on` across comma; content-word doubles; both-lower one-word contractions;
bare `it it's`; email/URL ITN (p19/p20); p41; unload; N203; N197; year ITN; five-word Cap; general
triple-any-word; `, okay` without unmarked evidence; one-word STUTTERS across a period.

## Consequences

`Like, I'm. I'm a senior engineer now today` → `Like, I'm a senior engineer now today.`;
`It's. It's the top two things today` → `It's the top two things today.`;
`Don't. Don't push that change today` → `Don't push that change today.`. Kept: `I will go. I will
go…`; `I'm I'm…` / `I'm... I'm…` / `It's It's…`; `So. So…` / `Yes. Yes…` / `I. I…`;
`O'Brien. O'Brien…` / `Bob's. Bob's…`. Polish 349/352 → **357/360** exact, edit distance 39 (0.30 % of 12854), regressions 0, keep 0/666.
Release `cleanup_budget` Cap Cap period median **0.474 ms** (1106-word; ceiling 2 ms). Capture, segmenting, decode, insert unchanged.
