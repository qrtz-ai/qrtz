# ADR 0243: Echo polish, an unmarked confirmation tag (", right" / ", correct") ends with "?"

- Status: accepted (echo cycle 126, 2026-11-06)
- Code: crates/qrtz-talk/src/cleanup.rs (`confirmation_tag` via `tag_question`, after imperative tags),
  packages/core/src/talk/cleanup.ts (`confirmationTag` / `CONFIRMATION_TAGS`), spec/vectors/talk/polish.json
  (+8 polish p229–p236; +14 keep k515–k528), polish-baseline.json, cleanup.json, tests/hardening126.rs,
  cleanup.hardening126.test.ts, tests/cleanup_budget.rs, tests/hardening60.rs (oracle)

## Context

ADR 0227 deferred positive / confirmation tags until real dictations showed them. Prod + dev talk history (711
entries, 2026-11-06) holds many ", right?" and one ", correct?" where Parakeet already wrote the mark from rising
intonation. When the same shape is unmarked (flat intonation, or a polish vector), `terminal_punctuation` was adding
"." — "That works for me, right." instead of "That works for me, right?".

Inverted positive tags ("is it", "do you", "will you" without an imperative opener) stay out: they are still a
cut-off question start. Direction lists that end ", right" after left/up/north/… must stay statements.

## Decision

`tag_question` also accepts a one-word tag that is exactly lowercase "right" or "correct" (`CONFIRMATION_TAGS`),
with at least one word before the comma and no `; : " ( )` in the sentence (same gates as ADR 0227). If the word
immediately before the comma is in `CONFIRMATION_NOT_BEFORE` (left/right/up/down/north/south/east/west/forward/back/…),
refuse — "Go up, down, left, right" and "Please turn left, right" keep ".".

No LLM, no network. Rejected this cycle: ", okay" / ", yeah" / ", huh" / ", eh" / ", no" (thin or ambiguous);
capitalized ", Right"; "the one on the left, right" (would need its own keep set against direction lists); changing a
mark Parakeet already wrote; inverted positive tags without an opener; year ITN; email/URL ITN; p41.

## Consequences

Unmarked confirmation tags get "?". Polish 225/228 → 233/236, edit distance stays 39 (0.46 % of 8422), regressions 0, keep 0/527. Release `cleanup_budget` confirmation-tag text median 0.197 ms (ceiling 2 ms). Capture, segmenting, decode, insert unchanged.
