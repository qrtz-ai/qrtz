# ADR 0241: Echo polish, a comma that sits only before a sentence mark is dropped

- Status: accepted (echo cycle 88, 2026-11-05), QRT-265
- Code: (see cycle 88 commit) crates/qrtz-talk/src/cleanup.rs (`dangling_comma_marks`, last step of `rule_cleanup`),
  packages/core/src/talk/cleanup.ts (`DANGLING_COMMA_MARK`, `danglingCommaMarks`), spec/vectors/talk/polish.json
  (+8 polish p213-p220, +12 keep k489-k500), polish-baseline.json, cleanup.json, tests/hardening88.rs,
  cleanup.hardening88.test.ts, tests/cleanup_budget.rs, tests/hardening60.rs (oracle), hardening83/84 expectations

## Context

After fillers go, real Parakeet hypotheses sometimes leave a comma sitting only before a sentence mark: "Yeah, also,
uh." typed "Yeah, also, .", and "…thing, um. Make that…" typed "…thing, . Make that…". ADR 0235 left mid-sentence
"So, um. Next" alone; the dangling `, .` shape itself is never written English.

## Decision

`dangling_comma_marks` runs last (after `comma_runs`) and loops to a fixed point. A `,`, optional `" ,"` pieces, ASCII
spaces/tabs, a run of `.`/`!`/`?`, then an ASCII blank or the end, becomes just the marks. Marks stuck to a word
("Yes, .NET", "1, .5") stay. Unicode blanks after the mark do not match (Rust = TS).

## Consequences

Two real prod dictations clean up; "So, um. Next we go" becomes "So. Next we go." Polish 208/212 → 216/220, regressions 0. Chain cost stays well under 2 ms (dangling text 0.174 ms median release on the box).
