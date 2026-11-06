## 2026-11-06 — Cycle 131 (≈10:51–11:05 PT, polish): Cap-leading two-word and three-word restart phrases (ADR 0247)

- **Landed:** widen `restart_phrase` — Title-case first copy ("Can you Can you") and three-word Cap
  restarts ("I don't know I don't know", "What are the What are the", "how I want How I want")
  collapse (ADR 0247). Rust + TS twin, polish +10, keep +7, cleanup.json, hardening131 + hardening60
  oracle, cleanup_budget probe.
- **Polish:** 259/262 → **269/272** exact, edit distance **39** (0.39 % of 10047), regressions **0**,
  keep **0/570**.
- **Latency:** release `cleanup_budget` restart-phrase median **0.246 ms** (993-word probe; ceiling 2 ms).
- **Rejected:** ", okay"/", yeah"/", huh"/", eh"/", no"; period ", right."; inverted tags;
  which/should openers; email/URL ITN; p41; unload; N203; N197 (Mac disk ~1.3 GB); year ITN; so/like
  STUTTERS; lowercase-second-copy phrase stutters without Cap.
- **SHAs:** code `a1e9ad49`, docs `5d73cc21`

# Hardening log

## 2026-11-06 — Cycle 130 (≈10:26–10:45 PT, polish): a mid-sentence two-word restart phrase collapses (ADR 0246)

- **Focus:** polish. origin/main at `7f54b676` (cycle 128 / ADR 0245). Mac online; prod+dev talk history (453+262;
  +4 new since cycle 127/129, including "as a As a").
- **Landed:** `restart_phrase` — "as a As a" / "in a In a" / "in the In the" / "it doesn't It doesn't" keep one
  copy (ADR 0246). Rust + TS twin, polish +8, keep +10, cleanup.json, hardening130 + hardening60 oracle,
  cleanup_budget +1 text.
- **Scores:** polish 251/254 → **259/262** exact, edit distance 39 (0.40 % of 9682), regressions 0; keep 0/563.
- **Rejected:** ", okay"/", yeah"/", huh"/", eh"/", no" (still 0 unmarked end hits; the only ", okay" hit was
  "Okay, okay."); email/URL ITN; p41; unload; N203; N197 (Mac disk ~1.5 GB); year ITN (k350); "make make" into
  STUTTERS.
- **Latency:** release `cleanup_budget` restart-phrase median **0.208 ms** (495-word baseline ~0.135 ms); ceiling 2 ms.
- **SHA:** `0489d5b3`

## 2026-11-06 — Cycle 128 (≈9:25–10:10 PT, polish): a sentence-initial filler after a period keeps the sentence mark (ADR 0245)

- **Focus:** polish. origin/main at `52ff5304` (cycle 127 / ADR 0244). Mac online; prod+dev talk history (453+261).
- **Landed:** `opening_filler_clause` — "Thanks. Um, can you…" / "thanks. Uh it says…" keep the period and capitalise
  the next word (ADR 0235's deferred comma form, now evidenced). Rust + TS twin, polish +10, keep −k496 +14,
  cleanup.json, hardening128 + hardening83/88 expects, cleanup_budget +1 text.
- **Scores:** polish 241/244 → **251/254** exact, edit distance 39 (0.42 % of 9293), regressions 0; keep 0/553.
- **Rejected:** ", okay"/", yeah"/", huh"/", eh"/", no" wideners (no unmarked end hit); email/URL ITN; p41; unload;
  N203 (still no free-relative false "?" in history); N197 (Mac disk ~6.2 GB — skipped mic switch).
- **Latency:** release `cleanup_budget` filler-clause median **0.216 ms** (495-word baseline ~0.14 ms); ceiling 2 ms.
- **SHA:** `b344c95b`

## 2026-11-06 — Cycle 127 (≈8:49–9:05 PT, polish): a confirmation sentence "…. Right" / "…. Correct" ends with "?" (ADR 0244)

- **Focus:** polish. origin/main at `cc752989` (cycle 126 / ADR 0243). Mac online; prod+dev talk history (453+258).
- **Landed:** `confirmation_sentence` — last sentence is lone Right/Correct after a prior sentence → "?"; also
  rewrites the recogniser's trailing "." on that shape. Direction prev-word refuse unchanged (ADR 0243 list).
- **Scores:** polish 233/236 → **241/244** exact, edit distance 39 (0.44 % of 8779), regressions 0; keep 0/540.
- **Latency:** release `cleanup_budget` confirmation-sentence medians ≪ 2 ms (period form ~0.12–0.28 ms).
- **Still failing (3):** p19 / p20 spoken email and URL (ADR 0223), p41 (ADR 0188 don't retry).
- **SHA:** `40ba7a02`

## 2026-11-06 — Cycle 126 (≈8:25–9:05 PT, polish): unmarked ", right" / ", correct" ends with "?" (ADR 0243)

- **Focus:** polish. origin/main at `b4a206c6` (cycle 125 / ADR 0242). Mac online; prod+dev talk history (453+258)
  re-scanned for confirmation tags. Many ", right?" and one ", correct?" already marked by Parakeet; unmarked form
  was getting "." from terminal_punctuation.
- **Landed:** "That works for me, right" → "That works for me, right?"; "You mean this one, correct" → "…, correct?".
  Direction lists ("Please turn left, right", "Go up, down, left, right") keep ".". Rust + TS twin, polish +8,
  keep +14, cleanup.json, hardening126 + hardening60 oracle, cleanup_budget +1 text.
- **Scores:** polish set 225 / 228, 39 chars → **233 / 236**, 39 chars (0.46 % of 8422), regressions 0; keep set 0 hits of 527.
- **Rejected:** ", okay"/", yeah"/", huh"/", eh"/", no"; capitalized ", Right"; "the one on the left, right";
  changing a mark Parakeet wrote; inverted positive tags; year ITN; email/URL ITN; p41; unload; N203 (still no
  free-relative false "?" in history); N197 (Mac disk ~7 GB, tauri/vitest up — skipped mic switch).
- **Suites:** qrtz-talk full suite (incl. `no_model_names_in_the_crate`); vitest talk twin; Note quick check;
  `cargo fmt -p qrtz-talk --check`; `bun run lint`.
- **Latency:** release cleanup_budget medians on the box: 495 words 0.123 ms; confirmation-tag text (700 words) **0.197 ms**; all under 2 ms.
- **Box:** shared target `/workspace/qrtz-core2/target`; worktree `/workspace/qrtz-echo-wt-126`.

## 2026-11-06 — Cycle 125 (≈7:39–8:05 PT, polish): spoken list "comma"/"period" become marks (ADR 0242)

- **Focus:** polish. origin/main still `77b4e922` (0.4.5 / cycle 88). Mac online; `echo-bench` on TTS spoken-list
  clips (Release helper + app `QRTZ_MODELS_DIR`) finally gave Parakeet emission evidence for p15.
- **Landed:** "Bring the charger comma the cable comma and the adapter period" → "Bring the charger, the cable, and
  the adapter."; same for eggs/milk/bread and Case B where commas are already ",". Rust + TS twin, polish +8,
  keep +14, cleanup.json, hardening125 + hardening60 oracle, cleanup_budget +2 texts.
- **Scores:** polish set 216 / 220, 58 chars → **225 / 228**, 39 chars (0.48 % of 8120), regressions 0; keep set
  0 hits of 513.
- **Rejected:** always-on spoken comma; bare final "period" without a list; year ITN; email/URL ITN; p41; unload;
  N203 (still no free-relative false "?" in history); N197 (device-switch not measured — tauri/vitest up, disk ~7 GB).
- **Suites:** qrtz-talk full suite green (incl. `no_model_names_in_the_crate`); vitest talk twin 707 passed in 21
  files; Note 130 passed in 19 files; `cargo fmt -p qrtz-talk --check` clean; `bun run lint` clean.
- **Latency:** release cleanup_budget medians on the box: 495 words 0.127 ms; spoken-list 0.146 ms; long list tail
  0.166 ms; all under 2 ms.
- **Box:** shared target `/workspace/qrtz-core2/target`; worktree `/workspace/qrtz-echo-wt-125`.

## 2026-11-05 — Cycle 88 (≈11:25–11:40 PT, polish): a comma before a sentence mark is dropped (ADR 0241, QRT-265)

- **Focus:** polish. Path review since `09a38c22` / `4f8903d5` (cycle 87): only `8af7f915` (oxfmt timing-report) landed;
  capture, VAD / segmenting, model config, drain / final pass, insert, pill unchanged. N197 / N203 still open; p15 /
  p19 / p20 / p41 still blocked.
- **Landed:** "Yeah, also, uh." types "Yeah, also." (was "Yeah, also, ."); "So, um. Next we go" types "So. Next we go."
  (was "So, . Next we go."). `dangling_comma_marks` last after `comma_runs`, fixed-point loop; ASCII-only blank after
  the mark. Rust + TS twin, polish +8, keep +12, cleanup.json, hardening88 + hardening60 oracle, hardening83/84
  expectations, cleanup_budget +1 text.
- **Scores:** polish set 208 / 212, 58 chars → **216 / 220**, 58 chars (0.74 % of 7859), regressions 0; keep set +12,
  0 hits of 499.
- **Rejected:** mark stuck to a word; Unicode whitespace after the mark; year ITN; email/URL ITN; p41; unload;
  spoken-comma without Parakeet evidence. N203 still open.
- **Suites:** qrtz-talk full suite green (incl. `no_model_names_in_the_crate`); vitest talk twin 694 passed in 20 files;
  Note 30 passed in 8 files; `cargo fmt -p qrtz-talk --check` clean.
- **Latency:** release cleanup_budget medians on the box: 495 words 0.120 ms; dangling text 0.174 ms; all under 2 ms.
- **Box:** shared target `/workspace/qrtz-core2/target`; worktree `/workspace/qrtz-echo88`.
