# ADR 0259: Echo polish, Cap Cap multi-word verbal across a period collapses

- Status: accepted (echo cycle 143, 2026-11-06)
- Code: crates/qrtz-talk/src/cleanup.rs (`restart_phrase` / `period_gap` / `is_verbal_contraction`),
  packages/core/src/talk/cleanup.ts (`restartPhrase` / `periodGap` / `isVerbalContraction`),
  spec/vectors/talk/polish.json (+8 polish p361–p368; keep retarget k613/k645 +8 k670–k677),
  polish-baseline.json, cleanup.json, tests/hardening143.rs, cleanup.hardening143.test.ts,
  tests/hardening136.rs / hardening140.rs (retarget Cap Cap MW period refuses), tests/hardening60.rs
  (allowed diff), tests/cleanup_budget.rs

## Context

ADR 0258 collapses Cap Cap **one-word** verbal contractions across a single period (`I'm. I'm`).
Cap Cap **multi-word** across a period stayed refused (`I will go. I will go`, and previously
`I don't really. I don't really`). Fresh Mac prod+dev talk history (2026-11-06, 453+443) still left
`I don't. I don't know how…` and `you can't. You can't…` after current cleanup — false segment cuts
between Cap Cap copies of a phrase that contains a verbal contraction. Cap Cap MW across ellipsis
already collapses via Cap-second (ADR 0252); the period gap was the remaining hole.

## Decision

Widen `restart_phrase` / `restartPhrase` two-/three-/four-word forms:

1. **Cap Cap / Cap-second across a single period** collapses when the second copy starts uppercase,
   `eq_ignore_ascii_case`, the gap is exactly spaces/tabs around one `.`, and **at least one word of
   the first copy** is a verbal / aux contraction (`is_verbal_contraction` — same filter as ADR 0258).
2. Cap Cap MW without a verbal contraction across a period stays (`I will go. I will go`,
   `I talk. I talk`, `We should ship. We should ship`).
3. Cap Cap one-word across spaces/ellipsis stays refused (ADR 0258). Cap Cap one-word verbal across
   a period still collapses (ADR 0258). Cap-then-lower / both-lower period-gap (ADR 0256) and
   both-lower contraction-headed spaces/ellipsis (ADR 0257) unchanged.
4. Dropping a Cap-starting second copy also strips the single period in the drop builder (so MW Cap
   Cap like `I don't. I don't` does not depend on `continued_dot`).

No LLM, no network. Rejected this cycle: Cap Cap MW without verbal across period; Cap Cap one-word
contraction across spaces/ellipsis; Cap Cap `So So` / `Where where`; `a An`; adding
are/is/on/like/why/so/where to STUTTERS; `hold on, hold on` across comma; content-word doubles;
both-lower one-word contractions; bare `it it's`; email/URL ITN (p19/p20); p41; unload; N203; N197;
year ITN; five-word Cap; general triple-any-word; `, okay` without unmarked evidence; one-word
STUTTERS across a period.

## Consequences

`I don't. I don't know how to do all this stuff today` → `I don't know how to do all this stuff today.`;
`you can't. You can't ship that without tests today` → `You can't ship that without tests today.`;
`I don't really. I don't really like this idea today` → `I don't really like this idea today.`.
Kept: `I will go. I will go…`; `I talk. I talk…`; `I'm I'm…` / `I'm... I'm…`; `So. So…`.
Polish 357/360 → **365/368** exact, edit distance 39 (0.30 % of 13151), regressions 0, keep 0/674.
Release `cleanup_budget` Cap Cap MW period median **0.830 ms** (1186-word; ceiling 2 ms). Capture,
segmenting, decode, insert unchanged.
