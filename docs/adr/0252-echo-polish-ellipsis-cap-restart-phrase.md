# ADR 0252: Echo polish, Cap-restart phrases across an ellipsis collapse

- Status: accepted (echo cycle 136, 2026-11-06)
- Code: crates/qrtz-talk/src/cleanup.rs (`restart_phrase` copy-gap + drop strip),
  packages/core/src/talk/cleanup.ts (`restartPhrase`), spec/vectors/talk/polish.json (+8 polish
  p305–p312; keep +8 k612–k619), polish-baseline.json, cleanup.json, tests/hardening136.rs,
  cleanup.hardening136.test.ts, tests/cleanup_budget.rs, tests/hardening60.rs (oracle)

## Context

ADR 0246–0250 collapse a mid-sentence Cap-restart phrase when copies are separated only by
spaces/tabs ("as a As a", "Can you Can you", "we could just have We could just have"). Fresh Mac
dev talk history (2026-11-06 ~10:46 PT) still typed a Cap restart with an ellipsis between copies:
"But um I don't really... I don't really like this" — filler removal leaves "I don't really... I
don't really like this", which stayed. A single `.` between copies is a real sentence boundary
("I'll use it. It needs to work") and must stay.

## Decision

Widen `restart_phrase` (still after `collapse_repeats`): the gap **between** the last word of the
first copy and the first word of the second may be spaces/tabs, or spaces/tabs around `...` /
`…` (U+2026). Within each copy, only spaces/tabs. A single `.` does not match. When the second
copy is dropped, strip the ellipsis with it. Same Cap-restart signature as ADR 0246–0250 (and
one-word Cap contractions / bare "it It's"). Spoken "scratch that" across an ellipsis stays
(ADR 0186). Proper-name Cap on the last word of the second copy stays ("new york... New York").

No LLM, no network. Rejected this cycle: ", okay"/", yeah"/", huh"/", eh"/", no" (still 0 unmarked
end hits); "why Why" / "is Is" / "like Like" / adding those to STUTTERS; "a An"; "there's a so
there's a" (lowercase second copy; ADR 0247 refuse); five-word Cap restart (0 hits); general
triple-any-word; email/URL ITN (p19/p20, ADR 0223); p41 (ADR 0188 don't retry); unload;
speculative N203; N197 mic-switch (Mac disk ~12 GB free but still disruptive); year ITN (k350);
collapsing lowercase-second phrase stutters across ellipsis without Cap evidence.

## Consequences

"I don't really... I don't really like this" → "I don't really like this."; "Can you... Can you
host the prototype on a server" → "Can you host the prototype on a server?"; "we could just
have... We could just have our configuration ready" → "We could just have our configuration
ready.". Kept: "I'll use it. It needs to work correctly today"; "I don't really. I don't really
like this idea"; "new york... New York looks nice today"; "scratch that... Scratch that for me
please"; "like a like a smooth fade across the board"; "Wait... Wait a minute please now"; prior
ADR 0248/0250/0251 keeps. Polish 301/304 → **309/312** exact, edit distance stays 39, regressions
0, keep 0/616. Release `cleanup_budget` ellipsis Cap-restart median **0.274 ms** (986-word;
ceiling 2 ms). Capture, segmenting, decode, insert unchanged.
