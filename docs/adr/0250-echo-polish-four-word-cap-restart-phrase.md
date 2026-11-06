# ADR 0250: Echo polish, four-word Cap-leading restart phrases collapse

- Status: accepted (echo cycle 134, 2026-11-06)
- Code: crates/qrtz-talk/src/cleanup.rs (`restart_phrase`, after `collapse_repeats`),
  packages/core/src/talk/cleanup.ts (`restartPhrase`), spec/vectors/talk/polish.json (+8 polish
  p289–p296; keep +8 k596–k603), polish-baseline.json, cleanup.json, tests/hardening134.rs,
  cleanup.hardening134.test.ts, tests/cleanup_budget.rs, tests/hardening60.rs (oracle)

## Context

ADR 0247 collapsed Cap-leading two- and three-word restart phrases ("Can you Can you", "I don't
know I don't know"). Fresh Mac prod+dev talk history (453 + 378, +39 since cycle 133) still shows a
four-word Cap restart that stayed typed: "Then we could just have We could just have our
configuration…". The three-word rule cannot fire on that shape — the equal span is four words.

Title-case proper-name runs of four words ("New York City Hall New York City Hall") stay out because
the last word of each copy starts uppercase. A second copy that starts lowercase ("we could just
have we could just have") is not the Parakeet Cap-restart shape. A Cap mid-copy ("I Don't") fails
the middle_ok check.

## Decision

Widen `restart_phrase` (still after `collapse_repeats`) with a four-word Cap-restart match on eight
adjacent words (spaces/tabs only):

- First word of the second copy starts uppercase; last word of each copy is lowercase
- The two middle words of each copy are lowercase or the pronoun "I" / "I'd" / "I'm" / "I've" /
  "I'll"
- eq_ignore_ascii_case across the four pairs
- Prefer four- then three- then two- then one-word when more than one could apply

No LLM, no network. Rejected this cycle: ", okay"/", yeah"/", huh"/", eh"/", no" wideners (still 0
unmarked end hits); "a An" article Cap (different lemma; Cap mid-sentence "An" is wrong without a
lowercasing pass); "why Why" / adding "why"|"is"|"on"|"like" to STUTTERS; "hold on, hold on" across
a comma (restart_phrase is spaces/tabs only); email/URL ITN (p19/p20, ADR 0223); p41 (ADR 0188 don't
retry); unload; speculative N203; N197 mic-switch; year ITN (k350).

## Consequences

"Then we could just have We could just have our configuration" → "Then we could just have our
configuration."; "What did they say What did they say about the plan" → "What did they say about
the plan?". Kept: both-lower four-word, Cap-last proper names, lowercase-second-copy, Cap-middle
"I Don't", unequal "…have something We could just have…", prior ADR 0246/47/48/49 keeps. Polish
285/288 → **293/296** exact, edit distance stays 39 (0.36 % of 10751), regressions 0, keep 0/600.
Release `cleanup_budget` four-word Cap probe median must stay ≪ 2 ms. Capture, segmenting, decode,
insert unchanged.
