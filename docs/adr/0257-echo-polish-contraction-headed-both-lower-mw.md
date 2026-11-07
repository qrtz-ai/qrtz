# ADR 0257: Echo polish, both-lower contraction-headed multi-word restart (spaces/ellipsis)

- Status: accepted (echo cycle 141, 2026-11-06)
- Code: crates/qrtz-talk/src/cleanup.rs (`restart_phrase` / `both_lower*` / `word_has_apos`),
  packages/core/src/talk/cleanup.ts (`restartPhrase` / `bothLower*`),
  spec/vectors/talk/polish.json (+8 polish p345–p352; keep +8 k652–k659; k638 retargeted),
  polish-baseline.json, cleanup.json, tests/hardening141.rs, cleanup.hardening141.test.ts,
  tests/hardening139.rs (refusal retarget), cleanup.hardening139.test.ts, tests/hardening60.rs
  (allowed diff), tests/cleanup_budget.rs

## Context

ADR 0255 Cap-then-lower multi-word collapses "How's it going how's it going" / "Is that is that"
when the first copy starts Cap. ADR 0256 both-lower multi-word collapses only across a single
period ("should be. should be"), deliberately refusing space-only both-lower so idioms and
hesitations like "like a like a" / "is that is that" stay. Fresh Mac prod talk history left
`what's it what's it return` after cleanup: both copies lower, first word a contraction, space
gap — a clear mid-phrase restart that Cap-then-lower cannot see (no Cap on the first copy) and
period-gap both-lower cannot see (no period).

## Decision

Widen `restart_phrase` / `restartPhrase` both-lower multi-word (2/3/4):

1. Keep ADR 0256: both-lower collapses across a single period regardless of apostrophe.
2. **ADR 0257:** both-lower also collapses across spaces/tabs or ellipsis when the **first word of
   each copy contains an apostrophe** (ASCII `'` or curly `’`) — contraction-headed restart.
   Examples: `what's it what's it`, `don't know don't know`, `how's it how's it`,
   `where's the where's the`, `don't really know don't really know`,
   `can't we just have can't we just have`, ellipsis form `what's it... what's it`.
3. Space-only both-lower **without** an apostrophe on the first word stays refused (`like a like a`,
   `is that is that`, `for the for the`, `what are the what are the`).
4. Both-lower **one-word** contractions stay refused (`it's it's`, `don't don't`, `we're we're`) —
   ADR 0248 / 0254 mixed-Cap only.
5. k638 (cycle 139 keep for space-only both-lower `how's it going how's it going`) is retargeted to
   non-contraction `what are the what are the options today` so the keep set still pins the
   space-only refuse; the contraction form is now a positive (p348 / Cap-then-lower p343 lineage).

No LLM, no network. Rejected this cycle: Cap Cap across period; Cap Cap `So So` / `Where where`;
`a An`; adding are/is/on/like/why/so/where to STUTTERS; `hold on, hold on` across comma; content-word
doubles; both-lower one-word contractions; bare `it it's` (ADR 0249 missed-comma risk); email/URL
ITN; p41; unload; N203; N197; year ITN; five-word Cap; general triple-any-word; `, okay` without
evidence; one-word STUTTERS period collapse; Cap Cap contraction across period (`I'm. I'm`).

## Consequences

`Does a set timeout return a variable or what's it what's it return today` →
`Does a set timeout return a variable or what's it return today.`;
`don't know don't know the answer today` → `Don't know the answer today.`;
`how's it how's it going for you today` → `How's it going for you today?`. Kept: space-only
non-contraction `like a like a…` / `is that is that…` / `for the for the…` / `what are the what are
the…`; both-lower one-word `it's it's…` / `don't don't…`. Polish 341/344 → **349/352** exact, edit
distance 39 (0.31 % of 12582), regressions 0, keep 0/656. Release `cleanup_budget`
contraction-headed median **0.464 ms** (1226-word; ceiling 2 ms). Capture, segmenting, decode,
insert unchanged.
