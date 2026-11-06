# ADR 0249: Echo polish, bare "it" before Cap "It's" restart collapses

- Status: accepted (echo cycle 133, 2026-11-06)
- Code: crates/qrtz-talk/src/cleanup.rs (`restart_phrase`, after `collapse_repeats`),
  packages/core/src/talk/cleanup.ts (`restartPhrase`), spec/vectors/talk/polish.json (+8 polish
  p281–p288; keep +12 k584–k595), polish-baseline.json, cleanup.json, tests/hardening133.rs,
  cleanup.hardening133.test.ts, tests/cleanup_budget.rs, tests/hardening60.rs (oracle)

## Context

ADR 0248 collapses equal Cap contraction restarts ("it's It's", "don't Don't") when both sides
carry an apostrophe and match `eq_ignore_ascii_case`. Fresh Mac talk history still shows a near
shape where the first token is bare lowercase "it" (no apostrophe) and the second is Cap "It's":
"Like it It's very hit or miss", "Uh it it's…" (both-lower), "before you commit it, it it's gotta".
ADR 0248 never fires on those — "it" has no apostrophe. Cap on the second token is the Parakeet
segment-restart signature; a missed comma before a lowercase "it's" stays lowercase.

## Decision

Widen `restart_phrase` (still after `collapse_repeats`, in the one-word slot after ADR 0248) with:

- Word A is exactly lowercase `it` (refuse `It` / `IT`)
- Word B is exactly Cap `It's` or curly-apos `It’s`
- Only spaces/tabs between (no newline)
- Drop A (and the whitespace before B); keep B as-is

Prefer three- then two- then one-word (0248 then 0249) matches.

**Cap-second required.** Both-lower "it it's" is rejected this cycle (missed-comma risk:
"with it it's hard to tell"). A Cap "It It's" is refused. No preposition guard: Cap alone is the
restart signal, and a preposition list would block the history hit "Like it It's…".

Rejected this cycle: both-lower "it it's"; ", okay"/", yeah"/", huh"/", eh"/", no" wideners;
"like Like" / adding so|like to STUTTERS; lowercase "like a like a"; "a An" article Cap restart
(different lemma; defer); "why Why" plain Cap allowlist expand; email/URL ITN (p19/p20, ADR 0223);
p41 (ADR 0188 don't retry); unload / N203 speculative / N197 mic-switch; year ITN (k350).

## Consequences

"Like it It's very hit or miss" → "Like It's very hit or miss."; "I think it It's broken already"
→ "I think It's broken already.". Kept: both-lower "it it's", Cap "It It's", "give it its due",
"hit It's…", "it Itself", prior ADR 0246/47/48 keeps. ADR 0248 equal-contraction still collapses.
Polish 277/280 → **285/288** exact, edit distance stays 39 (0.37 % of 10481), regressions 0, keep 0/592.
Release `cleanup_budget` it-before-It's probe median must stay ≪ 2 ms. Capture, segmenting, decode,
insert unchanged.
