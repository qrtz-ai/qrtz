# Hardening log

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
