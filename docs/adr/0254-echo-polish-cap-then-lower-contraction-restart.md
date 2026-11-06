# ADR 0254: Echo polish, Cap-then-lower one-word contraction restart collapses

- Status: accepted (echo cycle 138, 2026-11-06)
- Code: crates/qrtz-talk/src/cleanup.rs (`restart_phrase` one-word Cap form),
  packages/core/src/talk/cleanup.ts (`restartPhrase`), spec/vectors/talk/polish.json (+8 polish
  p321–p328; keep +8 k628–k635), polish-baseline.json, cleanup.json, tests/hardening138.rs,
  cleanup.hardening138.test.ts, tests/cleanup_budget.rs, tests/hardening60.rs (allowed diff)

## Context

ADR 0248 collapses a one-word Cap restart of a contraction when the first copy is lowercase and the
second is Cap ("it's It's"). Fresh Mac prod+dev talk history (2026-11-06 ~2:51 PM PT, 453+439)
still left the inverse shape: Cap then lowercase of the same contraction at sentence start
("What's what's the meeting pass code", "It's it's the top two things aren't fading"). Both-lower
("it's it's") and both-Cap ("It's It's") stay intentional keeps from ADR 0248.

## Decision

Widen the ADR 0248 one-word form so a contraction double also collapses when the first copy starts
uppercase and the second starts lowercase (`eq_ignore_ascii_case`, apostrophe required, spaces/tabs
or an ellipsis between — same `copy_gap_ok` as ADR 0252). Drop the second copy; keep the Cap first.
Both-Cap and both-lower still stay. Plain Cap doubles without an apostrophe stay with STUTTERS or
remain refused ("like Like", "why Why", "is Is", "Where where").

No LLM, no network. Rejected this cycle: Cap Cap "So So" (keep k581); "is Is" / "why Why" / "like
Like" / adding those or "so" to STUTTERS (ADRs 0248–0253); "a An" Cap-lemma; lowercase multi-word
restarts; "hold on, hold on" across a comma; content-word doubles ("make make", "campaign campaign");
both-lower contraction doubles; email/URL ITN (p19/p20, ADR 0223); p41 (ADR 0188 don't retry);
unload; speculative N203; N197 mic-switch; year ITN (k350); five-word Cap restart; general
triple-any-word.

## Consequences

"What's what's the meeting pass code" → "What's the meeting pass code?"; "It's it's the top two
things aren't fading now" → "It's the top two things aren't fading now."; "What's... what's the
meeting pass code now" → "What's the meeting pass code now?". Kept: "it's it's…", "It's It's…",
"don't don't…", "O'Brien O'Brien…", "Can't Can't…", "What's What's…", "Where where…", "make
make…", prior ADR 0248 lower-then-Cap polish and ADR 0253 keeps. Polish 317/320 → **325/328**
exact, edit distance 39 (0.33 % of 11791), regressions 0, keep 0/632. Release `cleanup_budget`
Cap-then-lower median **0.348 ms** (1185-word; ceiling 2 ms). Capture, segmenting, decode, insert
unchanged.
