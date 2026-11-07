## 2026-11-06 — Cycle 143 (≈5:52–6:20 PM PT, polish): Cap Cap MW verbal across a period (ADR 0259)

- **Focus:** polish. origin/main at `e6cdbfda` (cycle 142 / ADR 0258). Mac online; prod+dev talk history (453+443).
- **Landed:** Cap Cap / Cap-second multi-word across a single period when the first copy contains a
  verbal contraction — "I don't. I don't…" → "I don't…"; "you can't. You can't…" → "You can't…".
  Cap Cap MW without verbal stays. Rust + TS twin, polish +8, keep retarget k613/k645 +8, cleanup.json,
  hardening143 + hardening136/140 retarget + hardening60 oracle, cleanup_budget +1 text.
- **Scores:** polish 357/360 → **365/368** exact, edit distance 39 (0.30 % of 13151), regressions 0; keep 0/674.
- **Rejected:** Cap Cap MW without verbal across period; Cap Cap contraction across spaces/ellipsis;
  Cap Cap "So So" / "Where where"; "a An"; STUTTERS wideners; "hold on, hold on"; content doubles;
  both-lower one-word contractions; bare "it it's"; email/URL ITN; p41; unload; N203; N197; year ITN;
  five-word Cap; general triple-any-word; ", okay" without unmarked evidence; one-word STUTTERS across a period.
- **Still failing (3):** p19 / p20 spoken email and URL (ADR 0223), p41 (ADR 0188's "don't retry").
- **Latency:** release `cleanup_budget` Cap Cap MW period median **0.830 ms** (1186-word; ceiling 2 ms).
- **SHA:** code `d70e3428`, ADR `d1a9c15e`, LOG/tuning `825a2088`

## 2026-11-06 — Cycle 142 (≈5:19–5:40 PT, polish): Cap Cap verbal contraction across a period (ADR 0258)

- **Focus:** polish. origin/main at `6b44f54d` (cycle 141 / ADR 0257). Mac online; prod+dev talk history (453+443).
- **Landed:** Cap Cap one-word verbal contraction across a single period — "Like, I'm. I'm a senior…" →
  "Like, I'm a senior…"; verbal-contraction filter refuses O'Brien / Bob's / So. So. Rust + TS twin,
  polish +8, keep +10, cleanup.json, hardening142 + hardening60 oracle, cleanup_budget +1 text.
- **Scores:** polish 349/352 → **357/360** exact, edit distance 39 (0.30 % of 12854), regressions 0; keep 0/666.
- **Rejected:** Cap Cap non-contraction across period; Cap Cap contraction across spaces/ellipsis; Cap Cap
  "So So" / "Where where"; "a An"; adding are/is/on/like/why/so/where to STUTTERS; "hold on, hold on";
  content doubles; both-lower one-word contractions; bare "it it's"; email/URL ITN; p41; unload; N203;
  N197; year ITN; five-word Cap; general triple-any-word; ", okay" without unmarked evidence; one-word
  STUTTERS across a period.
- **Still failing (3):** p19 / p20 spoken email and URL (ADR 0223), p41 (ADR 0188's "don't retry").
- **Latency:** release `cleanup_budget` Cap Cap period median **0.474 ms** (1106-word; ceiling 2 ms).
- **SHA:** code `c0f8c75c`, ADR `3d580a1b`, LOG/tuning `13ebece2`

## 2026-11-06 — Cycle 141 (≈4:47–5:10 PM PT, polish): both-lower contraction-headed MW restart (ADR 0257)

- **Focus:** polish. origin/main at `00993647` (cycle 140 / ADR 0256). Mac online; prod+dev talk history (453+443).
- **Landed:** both-lower multi-word (2/3/4) also collapses across spaces/ellipsis when the first word of each
  copy contains an apostrophe (contraction-headed). Period-gap both-lower (0256) unchanged. Space-only
  non-contraction and both-lower one-word contractions stay. k638 retargeted to non-contraction
  `what are the what are the`. Rust + TS twin, polish +8, keep +8, cleanup.json, hardening141 +
  hardening139/60, cleanup_budget +1 text.
- **Scores:** polish 341/344 → **349/352** exact, edit distance 39 (0.31 % of 12582), regressions 0; keep 0/656.
- **Rejected:** Cap Cap across period; Cap Cap "So So" / "Where where"; "a An"; STUTTERS wideners;
  "hold on, hold on"; content-word doubles; both-lower one-word contractions; "it it's" (ADR 0249);
  email/URL ITN; p41; unload; N203; N197; year ITN; five-word Cap; general triple-any-word; ", okay";
  one-word STUTTERS period; Cap Cap contraction across period ("I'm. I'm").
- **Latency:** release `cleanup_budget` contraction-headed median **0.464 ms** (1226-word; ceiling 2 ms).
- **Still failing (3):** p19 / p20 spoken email and URL (ADR 0223), p41 (ADR 0188 don't retry).
- **SHA:** `ed22c3db` (code), `e58b12b5` (ADR), `4a0fc5ba` (LOG/tuning)

## 2026-11-06 — Cycle 140 (≈3:55–4:30 PM PT, polish): Cap-then-lower / both-lower across a period (ADR 0256)

- **Focus:** polish. origin/main at `e9def823` (cycle 139 / ADR 0255). Mac online; prod+dev talk history (453+443).
- **Landed:** `copy_gap_ok` allows a single `.` when the second copy starts lowercase; both-lower MW (2/3/4)
  collapses only across a single period; Cap-then-lower / Cap-second work across that gap. Layout `new line` /
  `new paragraph` stay. Leftover single period cleaned by `continued_dot` when raw has Cap. One-word STUTTERS
  period collapse deferred. Rust + TS twin, polish +8, keep +8, cleanup.json, hardening140 + hardening60,
  cleanup_budget +1 text.
- **Scores:** polish 333/336 → **341/344** exact, edit distance 39 (0.32 % of 12325), regressions 0; keep 0/648.
- **Rejected:** Cap Cap across period; space-only both-lower MW; Cap Cap "So So" / "Where where"; "a An";
  STUTTERS wideners; "hold on, hold on"; email/URL ITN; p41; unload; N203; N197; year ITN; five-word Cap;
  general triple-any-word; ", okay"; one-word STUTTERS period (deferred).
- **Latency:** release `cleanup_budget` period-gap median **0.437 ms** (1227-word; ceiling 2 ms).
- **Still failing (3):** p19 / p20 spoken email and URL (ADR 0223), p41 (ADR 0188 don't retry).
- **Note:** `hardening60` `generated_text_is_byte_identical_to_cycle_59` already fails on origin/main at
  `e9def823` (same `\u{2028}` / cue-arts fixture); `every_vector` green. Not introduced this cycle.
- **SHA:** `86eab151` (code), `83ee3be6` (ADR), `8042e815` (LOG)

## 2026-11-06 — Cycle 139 (≈3:21–3:35 PM PT, polish): Cap-then-lower multi-word restart collapses (ADR 0255)

- **Focus:** polish. origin/main at `3ebea9ae` (cycle 138 / ADR 0254). Mac online; prod+dev talk history (453+443).
- **Landed:** Cap-then-lower widen of ADR 0246–0250 two-/three-/four-word `restart_phrase` — "Is that is that" /
  "You have to you have to" / "How's it going how's it going" (and ellipsis form) drop the second copy. Cap-second
  stays. Both-lower multi-word stays. Rust + TS twin, polish +8, keep +8 (k568/k583 Cap-then-lower → both-lower),
  cleanup.json, hardening139 + hardening131/60, cleanup_budget +1 text.
- **Scores:** polish 325/328 → **333/336** exact, edit distance 39 (0.32 % of 12036), regressions 0; keep 0/640.
- **Rejected:** Cap Cap "So So"; "is Is"/"why Why"/"like Like"; adding are/is/on/like/why/so/where to STUTTERS;
  "a An"; lowercase multi-word restarts; "hold on, hold on" across comma; content-word doubles; both-lower
  contraction doubles; email/URL ITN; p41; unload; N203; N197; year ITN; five-word Cap restart; general
  triple-any-word; ", okay"/, yeah/, huh/, eh/, no.
- **Latency:** release `cleanup_budget` Cap-then-lower multi-word median **0.385 ms** (1347-word; ceiling 2 ms).
- **Still failing (3):** p19 / p20 spoken email and URL (ADR 0223), p41 (ADR 0188 don't retry).
- **SHA:** `f1f1c1b3` (code), `1eaec066` (docs)

## 2026-11-06 — Echo cycle 138 (ADR 0254 Cap-then-lower contraction)

- **Landed:** Cap-then-lower one-word contraction doubles collapse ("What's what's", "It's it's";
  ellipsis gap ok). ADR 0248 lower-then-Cap still collapses; both-Cap / both-lower stay.
- **Scores:** polish 317/320 → **325/328** exact, edit distance 39, regressions 0, keep 0/632.
- **Latency:** release `cleanup_budget` Cap-then-lower median **0.348 ms** (1185-word; ceiling 2 ms).
- **Rejected:** Cap Cap "So So"; "is Is" / "why Why" / "like Like" / adding those or "so" to STUTTERS;
  "a An"; lowercase multi-word restarts; "hold on, hold on" across comma; content-word doubles;
  both-lower contraction doubles; email/URL ITN; p41; unload; N203; N197; year ITN.
- **SHA:** code `3a5ccceb`; docs `6de1db84`

## 2026-11-06 — Cycle 137 (≈1:57–2:20 PM PT, polish): triple+ "on" / "like" stutter collapses (ADR 0253)

- **Focus:** polish. origin/main at `aed2acad` (cycle 136 / ADR 0252). Mac online; prod+dev talk history (453+424).
- **Landed:** extend `TRIPLE_STUTTERS` / `collapses` / TS REPEATS — a run of 3+ space/tab-separated "on" or "like"
  (any case) collapses to the first copy (ADR 0253; same allowlist as ADR 0251's "are"). Doubles stay; never across
  commas; not added to STUTTERS. Rust + TS twin, polish +8, keep +8, cleanup.json, hardening137 + hardening60 oracle,
  cleanup_budget +1 text.
- **Scores:** polish 309/312 → **317/320** exact, edit distance 39 (0.34 % of 11490), regressions 0; keep 0/624.
- **Rejected:** Cap Cap "So So" (k581); "so"/"on"/"like"/"is"/"why" into STUTTERS; lowercase multi-word restarts;
  em-dash Cap-restart (0 hits); ", okay"/", yeah"/", huh"/", eh"/", no"; email/URL ITN; p41; unload; N203; N197;
  year ITN; five-word Cap; general triple-any-word.
- **Latency:** release `cleanup_budget` triple on/like median **0.348 ms** (1264-word; ceiling 2 ms).
- **SHA:** code `a8a6d784`; docs `3b3deceb`

## 2026-11-06 — Cycle 136 (≈1:26–1:45 PM PT, polish): Cap-restart phrases across an ellipsis collapse (ADR 0252)

- **Focus:** polish. origin/main at `c9c83656` (cycle 135 / ADR 0251). Mac online; prod+dev talk history (453+406;
  +28 since cycle 135 tip scan).
- **Landed:** `restart_phrase` copy-gap allows `...` / `…` between Cap-restart copies (spaces/tabs within a copy;
  single `.` stays a sentence boundary). Rust + TS twin, polish +8, keep +8, cleanup.json, hardening136 +
  hardening60 oracle, cleanup_budget +1 text.
- **Scores:** polish 301/304 → **309/312** exact, edit distance 39 (0.35 % of 11244), regressions 0; keep 0/616.
- **Rejected:** ", okay"/", yeah"/", huh"/", eh"/", no"; "why Why"/"is Is"/"like Like"/STUTTERS wideners; "a An";
  "there's a so there's a"; five-word Cap; general triple-any-word; email/URL ITN; p41; unload; N203; N197; year ITN.
- **Latency:** release `cleanup_budget` ellipsis Cap-restart median **0.274 ms** (986-word; ceiling 2 ms).
- **SHA:** `da880a3b` (code); docs `81569187`

## 2026-11-06 — Cycle 135 (≈12:51–1:20 PT, polish): triple+ "are" stutter collapses (ADR 0251)

- **Landed:** extend `collapses` / TS REPEATS — a run of 3+ space/tab-separated "are" (any case)
  collapses to the first copy (ADR 0251). Doubles stay; never across commas; "are" not added to
  STUTTERS. Rust + TS twin, polish +8, keep +8, cleanup.json, hardening135 + hardening60 oracle,
  cleanup_budget probe.
- **Polish:** 293/296 → **301/304** exact, edit distance **39**, regressions **0**, keep **0/608**.
- **Latency:** release `cleanup_budget` triple-are median **0.247 ms** (865-word probe; ceiling 2 ms).
- **Rejected:** "are" into STUTTERS; "on"/"is"/"why"/"like" STUTTERS; ", okay"/", yeah"/", huh"/", eh"/", no";
  email/URL ITN; p41; unload; N203; N197; year ITN; five-word Cap restart (0 hits); general triple-any-word.
- **SHAs:** code `6edb62f8`, docs `1f80b113`

## 2026-11-06 — Cycle 134 (≈12:18–12:40 PT, polish): four-word Cap restart phrase (ADR 0250)

- **Landed:** widen `restart_phrase` — mid-sentence Cap-leading four-word restart phrases collapse
  ("we could just have We could just have") (ADR 0250). Prefer four- then three- then two- then
  one-word. Rust + TS twin, polish +8, keep +8, cleanup.json, hardening134 + hardening60 oracle,
  cleanup_budget probe.
- **Polish:** 285/288 → **293/296** exact, edit distance **39**, regressions **0**, keep **0/600**.
- **Latency:** release `cleanup_budget` four-word Cap median **0.247 ms** (1027-word probe; ceiling 2 ms).
- **Rejected:** ", okay"/", yeah"/", huh"/", eh"/", no"; "a An"; "why Why" / STUTTERS wideners;
  "hold on, hold on" across comma; email/URL ITN; p41; unload; N203; N197; year ITN.
- **SHAs:** code `d6fd4b4c`, docs `7dcbf81a`

## 2026-11-06 — Cycle 133 (≈11:55–12:20 PT, polish): bare "it" before Cap "It's" (ADR 0249)

- **Landed:** widen `restart_phrase` — mid-sentence bare lowercase "it" before Cap "It's" /
  curly-apos form drops "it" and keeps Cap (ADR 0249). Both-lower "it it's" refused (missed-comma).
  Rust + TS twin, polish +8, keep +12, cleanup.json, hardening133 + hardening60 oracle,
  cleanup_budget probe.
- **Polish:** 277/280 → **285/288** exact, edit distance **39**, regressions **0**, keep **0/592**.
- **Latency:** release `cleanup_budget` it-before-It's median **0.225 ms** (ceiling 2 ms).
- **Rejected:** both-lower "it it's"; ", okay"/", yeah"/", huh"/", eh"/", no"; like Like / so|like
  STUTTERS; lowercase "like a like a"; "a An"; "why Why"; email/URL ITN; p41; unload; N203; N197;
  year ITN.
- **SHAs:** code `20ac6c3c`, docs `8a0f025b`

## 2026-11-06 — Cycle 132 (≈11:19–11:50 PT, polish): one-word Cap contraction restart (ADR 0248)

- **Landed:** widen `restart_phrase` — mid-sentence Cap restart of a contraction ("it's It's",
  "don't Don't", "can't Can't") collapses (ADR 0248). Rust + TS twin, polish +8, keep +10,
  cleanup.json, hardening132 + hardening60 oracle, cleanup_budget probe.
- **Polish:** 269/272 → **277/280** exact, edit distance **39** (0.38 % of ~103xx), regressions **0**,
  keep **0/580**.
- **Latency:** release `cleanup_budget` Cap-contraction median **0.228 ms** (782-word probe; ceiling 2 ms).
- **Rejected:** ", okay"/", yeah"/", huh"/", eh"/", no"; like Like / so|like STUTTERS; lowercase
  "like a like a"; email/URL ITN; p41; unload; N203; N197 (mic-switch still disruptive; Mac ~16 GB
  free); year ITN.
- **SHAs:** code `cf0ae0ea`, docs `fcac7230`

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
