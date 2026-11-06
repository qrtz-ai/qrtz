# ADR 0244: Echo polish, a confirmation tag said as its own sentence ("…. Right" / "…. Correct") ends with "?"

- Status: accepted (echo cycle 127, 2026-11-06)
- Code: crates/qrtz-talk/src/cleanup.rs (`confirmation_sentence` via `terminal_punctuation`, after
  `tag_question`), packages/core/src/talk/cleanup.ts (`confirmationSentence`), spec/vectors/talk/polish.json
  (+8 polish p237–p244; keep −k521 +14 k529–k542), polish-baseline.json, cleanup.json, tests/hardening127.rs,
  cleanup.hardening127.test.ts, tests/cleanup_budget.rs, tests/hardening60.rs (oracle)

## Context

ADR 0243 landed unmarked mid-sentence confirmation tags (", right" / ", correct") but kept the sentence-break form
as a refusal: "Just to confirm, all of this will automatically update. Right" typed "…. Right.". Prod talk history
(453) now holds that exact unmarked case, and another where rising intonation already wrote "…. Right?". When a
pause makes the recogniser start a new sentence on the tag, `terminal_punctuation` was adding or leaving ".".

Direction lists that end a prior sentence with left/up/north/… before a lone "Right" must stay statements
("Turn left. Right.").

## Decision

`confirmation_sentence` accepts a last sentence that is exactly one `CONFIRMATION_TAGS` word (any case; after a
sentence end the recogniser capitalises it), with a prior sentence in the same dictation, and with the previous
sentence's last word not in `CONFIRMATION_NOT_BEFORE`. It fires in two places in `terminal_punctuation`:

1. Unmarked end (`…. Right`) — push "?" instead of ".".
2. Recogniser already wrote a single "." (`…. Right.`) — rewrite that "." to "?". Does not touch "Right?", "Right!",
   or "Right…".

No LLM, no network. Rejected this cycle: mid-dictation ". Right." clauses (only the final sentence is marked, same
as tag_question); ", okay" / ", yeah" / ", huh" / ", eh" / ", no"; year ITN (k350); email/URL ITN (p19/p20); p41;
unload; speculative N203 without history evidence.

## Consequences

Unmarked and period-marked confirmation sentences get "?". Polish 233/236 → 241/244, edit distance stays 39
(0.44 % of 8779), regressions 0, keep 0/540. Release `cleanup_budget` confirmation-sentence text median ~0.12–0.28 ms
(ceiling 2 ms). Capture, segmenting, decode, insert unchanged.
