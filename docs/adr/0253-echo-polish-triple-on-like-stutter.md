# ADR 0253: Echo polish, triple+ "on" / "like" stutter collapses

- Status: accepted (echo cycle 137, 2026-11-06)
- Code: crates/qrtz-talk/src/cleanup.rs (`TRIPLE_STUTTERS` / `collapses`, after STUTTERS /
  COMMA_STUTTERS), packages/core/src/talk/cleanup.ts (`TRIPLE_STUTTERS` in REPEATS callback),
  spec/vectors/talk/polish.json (+8 polish p313–p320; keep +8 k620–k627), polish-baseline.json,
  cleanup.json, tests/hardening137.rs, cleanup.hardening137.test.ts, tests/cleanup_budget.rs,
  tests/hardening60.rs (oracle)

## Context

ADR 0251 collapsed a run of three or more "are" because doubles are real English ("you are Are
you") but triples are never intentional. Fresh Mac prod+dev talk history (2026-11-06 ~1:56 PM PT,
453+424) still left two of the same shape:

- "on on On the replay modal" (direct ASR triple; doubles like "turn it on on Friday" stay)
- "like like uh like to ping" → after filler drop, "like like like to ping" (discourse stutter;
  doubles like "I was like like whatever" and "like Like it or not" stay)

Neither word belongs in STUTTERS (ADR 0247 / cycle 135 refused adding "on"/"like").

## Decision

Extend the ADR 0251 allowlist to `TRIPLE_STUTTERS = ["are", "on", "like"]`. A run of **three or
more** space/tab-separated copies (any ASCII case via eq_ignore_ascii_case / toLowerCase) collapses
to the first copy. Doubles stay. Never across commas. Do **not** add "on" or "like" to STUTTERS.

No LLM, no network. Rejected this cycle: Cap Cap "So So" (keep k581 expects the double to stay;
"so so" mediocre); adding "so"/"on"/"like"/"is"/"why" to STUTTERS; lowercase multi-word restarts
("the starting deck the starting deck", "what's it what's it" — ADR 0247 keep); em-dash Cap-restart
(0 history hits); ", okay"/", yeah"/", huh"/", eh"/", no"; email/URL ITN (p19/p20, ADR 0223); p41
(ADR 0188 don't retry); unload; speculative N203; N197 mic-switch; year ITN (k350); five-word Cap
restart; general triple-any-word (breaks "No no no" / "Wait wait wait" / "yeah yeah yeah").

## Consequences

"on on On the replay modal" → "On the replay modal."; "turn it on on on Friday please" → "Turn it
on Friday please."; "like like like to ping an API" → "Like to ping an API."; "easiest for you to
like like uh like to ping" → "Easiest for you to like to ping.". Kept: "turn it on on Friday
please", "I was like like whatever about it", "like Like it or not…", "like a like a smooth fade…",
"on, on the other hand…", "why Why did we…", prior ADR 0251 are-double keeps. Polish 309/312 →
**317/320** exact, edit distance 39 (0.34 % of 11490), regressions 0, keep 0/624. Release
`cleanup_budget` triple on/like median **0.348 ms** (1264-word; ceiling 2 ms). Capture, segmenting,
decode, insert unchanged.
