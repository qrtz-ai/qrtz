//! Grant helpers derived only from package manifests (ADR 0212, QRT-367).
//!
//! Package and model names live in `modules/*/qrtz.json` (`extension.reads`, `extension.packageReads`,
//! `extension.readsViaMessage`). This module must not hard-code any of those names as string literals —
//! `GRANT_LOGIC` in `offline_graph.rs` scans this file.
use crate::qrtz_json::PackageManifest;

/// True when a bundled extension declares at least one package read (`extension.packageReads`).
/// Replaces the old `BUNDLED_PACKAGE_READS` name allowlist.
pub fn bundled_has_package_reads(extension: &str) -> bool {
    crate::bundled::declared_reads()
        .iter()
        .any(|(name, _)| name == extension)
}

/// True when a bundled extension's package reads reach the helper in a host message
/// (`extension.readsViaMessage`), never as `QRTZ_READ_MODELS`. That env makes the helper use no other dir;
/// extensions that also load speech sets from `<data>/models/` must keep it unset.
pub fn bundled_reads_via_message(extension: &str) -> bool {
    for (_, raw) in crate::bundled::package_manifests() {
        let Ok(m) = PackageManifest::parse(raw.as_bytes()) else {
            continue;
        };
        if m.name != extension {
            continue;
        }
        return m.extension.as_ref().map(|e| e.reads_via_message).unwrap_or(false);
    }
    false
}
