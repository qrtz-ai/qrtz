# ADR 0242: Echo polish, spoken list "comma" / "period" become "," / "."

- Status: accepted (echo cycle 125, 2026-11-06)
- Code: crates/qrtz-talk/src/cleanup.rs (`spoken_list_punct`, after `final_mark`), packages/core/src/talk/cleanup.ts
  (`spokenListPunct`), spec/vectors/talk/polish.json (+8 polish p221–p228; p15 now exact), +14 keep k501–k514,
  polish-baseline.json, cleanup.json, tests/hardening125.rs, cleanup.hardening125.test.ts, tests/cleanup_budget.rs,
  tests/hardening60.rs (oracle)

## Context

Polish p15 asks "Bring the charger comma the cable comma and the adapter period" to type as a real list. Cycles
57–88 left it blocked until Mac evidence showed how the on-device recogniser writes those words. On 2026-11-06,
`echo-bench` (Release helper, app model dir, TTS of the spoken form) returned:

- "Bring the charger, the cable, and the adapter period." (commas already marks; trailing "period" still a word)
- "We need eggs comma milk comma and bread period." (both words left)

Keep phrases ("Add a comma after the name.", "The trial period ends Friday.", "This is a period of growth.") stayed
exact. Bare final "period" without a list (school / grace period, "a period") stays out of scope this cycle.

## Decision

`spoken_list_punct` runs after `final_mark`. It fires only when the dictation (a trailing `.`/`?`/`!` aside) ends in
lowercase " period", the word before that passes the same plain-word / `MARK_NOUN_BEFORE` guards as ADR 0189, and
either:

- **(A)** the body holds lowercase " comma and " (Oxford spoken list): rewrite each whole-word "comma" whose previous
  word is not in `COMMA_NOUN_BEFORE`; if none rewrite, no-op.
- **(B)** the body already holds ASCII `,` (recogniser turned the commas into marks): drop only the trailing spoken
  "period".

No LLM, no network, one linear scan. Rejected: always-on "comma"→"," (false on "the word comma means…"); bare final
"period" without a list (grace / lunch / Tudor period); email/URL ITN; p41 list retry.

## Consequences

p15 becomes exact; polish 216/220 → 225/228, edit distance 58 → 39, regressions 0, keep 0/513. Release
`cleanup_budget` medians on the box stay well under 2 ms (spoken-list text 0.146 ms; long list tail 0.166 ms).
