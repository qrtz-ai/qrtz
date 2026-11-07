# ADR 0260: Echo polish, Cap Cap pronoun "I" across a period collapses

- Status: accepted (echo cycle 145, 2026-11-06)
- Code: crates/qrtz-talk/src/cleanup.rs (`restart_phrase` / `period_gap`),
  packages/core/src/talk/cleanup.ts (`restartPhrase` / `periodGap`),
  spec/vectors/talk/polish.json (+8 polish p369–p376; keep −k668 +10 k678–k687),
  polish-baseline.json, cleanup.json, tests/hardening145.rs, cleanup.hardening145.test.ts,
  tests/hardening142.rs (retarget I. I refuse), tests/hardening60.rs (allowed diff),
  tests/cleanup_budget.rs

## Context

ADR 0258 collapses Cap Cap **verbal contractions** across a single period (`I'm. I'm`) and
keeps Cap Cap without a verbal (`So. So`, `Yes. Yes`, `I. I`, `Where. Where`). ADR 0256 deferred
one-word STUTTERS across a period when tried via re-running `collapse_repeats` after
`continued_dot` (false hits on the hardening60 corpus). Fresh Mac prod+dev talk history
(2026-11-06, still 453+443) leaves a real Cap Cap pronoun restart: `But I. I just feel like a
moron` — a false segment cut between two Cap copies of `I`. `continued_dot` cannot drop the
period (second copy starts Cap), so the space-adjacent STUTTERS path never sees it.

## Decision

Widen `restart_phrase` / `restartPhrase` one-word form (same drop-builder period strip as
ADR 0258 / 0259):

1. **Cap Cap across a single period** also collapses when both copies are the pronoun `I`
   (`eq_ignore_ascii_case`, both start uppercase, gap is exactly spaces/tabs around one `.`,
   no apostrophe). Plain `I` only — `I'm` / `I'd` stay with ADR 0258.
2. Cap Cap non-`I` across a period stays (`So. So`, `What. What`, `The. The`, `Will. Will`,
   `A. A`, `Where. Where`, `We. We`, …). Full-STUTTERS period collapse stays refused (name /
   grade / discourse FPs).
3. Cap Cap `I` across spaces/ellipsis stays refused here (`I... I`); space `I I` still collapses
   via [`collapse_repeats`] / STUTTERS as before.
4. Cap Cap MW without verbal (ADR 0259 refuse) and Cap Cap verbal across spaces/ellipsis
   (ADR 0258 refuse) unchanged.

No LLM, no network. Rejected this cycle: Cap Cap MW without verbal across period; Cap Cap
one-word contraction across spaces/ellipsis; Cap Cap `So So` / `Where where`; full-STUTTERS
period collapse (`What. What`, `Will. Will`, `The. The`, `A. A`); `a An`; adding
are/is/on/like/why/so/where to STUTTERS; `hold on, hold on` across comma; content-word doubles;
both-lower one-word contractions; bare `it it's`; email/URL ITN (p19/p20); p41; unload; N203;
N197; year ITN; five-word Cap; general triple-any-word; `, okay` without unmarked evidence.

## Consequences

`But I. I just feel like a moron today` → `But I just feel like a moron today.`;
`I. I think that works for the team today` → `I think that works for the team today.`.
Kept: `So. So…` / `What. What…` / `Will. Will…` / `The. The…` / `A. A…` / `I... I…` /
`I talk. I talk…` / `I'm I'm…`. Polish 365/368 → **373/376** exact, edit distance 39
(0.29 % of 13520), regressions 0, keep 0/683. Release `cleanup_budget` Cap Cap I period
median **0.861 ms** (1187-word; ceiling 2 ms). Capture, segmenting, decode, insert unchanged.
