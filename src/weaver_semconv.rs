//! OpenTelemetry Weaver semantic-convention registry — a real `Admit` boundary.
//!
//! This module wires the real, on-disk `weaver` binary (OpenTelemetry Weaver,
//! `weaver registry check`) into this crate's `Admit` machinery, exactly the way
//! [`crate::ocel::LinkedOcel`] wires OCEL validation: `Raw` evidence in, a real
//! subprocess call decides admissibility, `Admission`/`Refusal` out.
//!
//! ## Why this boundary is real, and what it honestly is not
//!
//! Weaver's own `check --help` documents its internal pipeline as four steps:
//! load, parse, resolve references/extends, check Rego policy. None of those
//! four steps is independently invocable through the real CLI (`resolve` is a
//! deprecated subcommand pointing at `generate`/`package`, not a standalone
//! gate) — `check`, `generate`, `stats`, `diff`, and `package` each *privately*
//! re-run all four as one atomic step. So this boundary models weaver's whole
//! pipeline as a single opaque `Raw → Admitted` gate (one subprocess call),
//! matching what the binary actually exposes. It does **not** attempt to carve
//! `Raw → Parsed → Admitted` into two separately-invokable weaver-side stages —
//! doing so would misrepresent the CLI's real shape.
//!
//! The refusal reason is the real named top-level discriminant weaver's own
//! `--diagnostic-format json` output carries in each diagnostic's `error`
//! object (e.g. `"Resolver"`, `"Semconv"`, `"LegacyRegistryManifest"`) — not an
//! invented or bare "InvalidInput" string. Diagnostic kinds this module has not
//! seen are preserved verbatim via [`WeaverRefusalReason::Other`] rather than
//! dropped, so a real refusal is never silently coerced into a fixed enum that
//! cannot represent it.
//!
//! Structure-only, as this crate always is: `admit` shells out to weaver and
//! classifies its exit code and diagnostic kind. It performs no semantic
//! convention checking of its own.

use std::path::Path;
use std::process::Command;

use crate::admission::{Admission, Refusal};
use crate::evidence::Evidence;
use crate::state::Raw;
use crate::witness::WitnessFamily;

witness_marker!(
    /// The OpenTelemetry Weaver semantic-convention registry format.
    WeaverSemconv,
    "weaver-semconv",
    WitnessFamily::Standard,
    "OTel Weaver Semantic Convention Registry",
    None
);

/// A raw, unvalidated pointer to a semantic-convention registry on disk —
/// a local folder or a `weaver`-recognized Git URL, exactly what `weaver
/// registry check -r <REGISTRY>` accepts.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WeaverRegistryInput {
    /// The path or Git URL passed to `weaver registry check -r`.
    pub registry: String,
    /// Path to the `weaver` binary to invoke. Callers running against a build
    /// that is not on `$PATH` (as of this writing there is no `weaver` on
    /// `$PATH`; the only real binary on this machine is
    /// `~/chicago-tdd-tools/target/debug/weaver`) must set this explicitly.
    pub weaver_bin: String,
}

impl WeaverRegistryInput {
    /// Points at `registry` using a `weaver` binary already resolvable by name
    /// (e.g. on `$PATH`).
    pub fn new(registry: impl Into<String>) -> Self {
        Self {
            registry: registry.into(),
            weaver_bin: "weaver".to_string(),
        }
    }

    /// Points at `registry` using an explicit path to the `weaver` binary.
    pub fn with_binary(registry: impl Into<String>, weaver_bin: impl Into<String>) -> Self {
        Self {
            registry: registry.into(),
            weaver_bin: weaver_bin.into(),
        }
    }
}

/// A registry that `weaver registry check` accepted (exit code 0). Carries the
/// raw diagnostics JSON (warnings, if any) alongside the input, since exit 0
/// can still carry non-fatal warning diagnostics — see the live `check` output
/// captured during this integration's verification, which passed with
/// `DeprecatedSyntaxInRegistryManifest` and `InvalidExampleWarning` warnings
/// present.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeaverAdmittedRegistry {
    /// The registry pointer that was checked.
    pub input: WeaverRegistryInput,
    /// The raw `--diagnostic-format json` output weaver produced (may be an
    /// empty array, or contain `Warning`-severity diagnostics).
    pub diagnostics_json: String,
}

/// The named reason `weaver registry check` refused a registry, taken verbatim
/// from the top-level key of the first diagnostic's `error` object in
/// weaver's own `--diagnostic-format json` output (e.g. `Resolver`, `Semconv`,
/// `LegacyRegistryManifest` — all confirmed against the real binary during
/// this integration's verification).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeaverRefusalReason {
    /// The named diagnostic kind weaver itself reported, plus the full
    /// diagnostic JSON it was extracted from.
    Named {
        /// The diagnostic's top-level `error` discriminant, e.g. `"Resolver"`.
        kind: String,
        /// The full `--diagnostic-format json` array weaver produced.
        diagnostics_json: String,
    },
    /// weaver exited non-zero but produced no parseable JSON diagnostic (e.g.
    /// the binary itself could not run, or emitted non-JSON output). Carries
    /// the raw stderr/stdout weaver produced, verbatim, so the failure is
    /// never silently discarded.
    ProcessFailure {
        /// weaver's process exit status, formatted (e.g. `"exit status: 1"`).
        status: String,
        /// Combined stdout+stderr weaver produced.
        output: String,
    },
}

/// The `Admit` boundary for [`WeaverSemconv`]: shells out to the real `weaver`
/// binary's `registry check` subcommand and classifies the result.
///
/// See the module-level docs for why this models weaver's whole
/// load/parse/resolve/policy pipeline as a single opaque gate rather than as
/// multiple `Evidence` transitions.
pub enum WeaverCheckAdmit {}

impl crate::admission::Admit for WeaverCheckAdmit {
    type Raw = WeaverRegistryInput;
    type Admitted = WeaverAdmittedRegistry;
    type Reason = WeaverRefusalReason;
    type Witness = WeaverSemconv;

    fn admit(
        raw: Evidence<WeaverRegistryInput, Raw, WeaverSemconv>,
    ) -> Result<
        Admission<WeaverAdmittedRegistry, WeaverSemconv>,
        Refusal<WeaverRefusalReason, WeaverSemconv>,
    > {
        let input = raw.value;

        let output = match Command::new(&input.weaver_bin)
            .arg("registry")
            .arg("check")
            .arg("-r")
            .arg(&input.registry)
            .arg("--diagnostic-format")
            .arg("json")
            .output()
        {
            Ok(output) => output,
            Err(io_err) => {
                return Err(Refusal::new(WeaverRefusalReason::ProcessFailure {
                    status: "failed to spawn weaver".to_string(),
                    output: io_err.to_string(),
                }));
            }
        };

        let combined = format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );

        if output.status.success() {
            return Ok(Admission::new(WeaverAdmittedRegistry {
                input,
                diagnostics_json: extract_json_array(&combined).unwrap_or_default(),
            }));
        }

        let diagnostics_json = extract_json_array(&combined);
        let kind = diagnostics_json
            .as_deref()
            .and_then(first_diagnostic_error_kind);

        match (kind, diagnostics_json) {
            (Some(kind), Some(diagnostics_json)) => Err(Refusal::new(WeaverRefusalReason::Named {
                kind,
                diagnostics_json,
            })),
            _ => Err(Refusal::new(WeaverRefusalReason::ProcessFailure {
                status: format!("{}", output.status),
                output: combined,
            })),
        }
    }
}

/// Extracts the top-level `[...]` JSON array weaver prints amid its human
/// log lines (weaver interleaves plain-text progress lines before the JSON
/// diagnostics array; this finds the array by its bracket balance rather than
/// assuming it is the only thing on stdout).
fn extract_json_array(combined: &str) -> Option<String> {
    let start = combined.find('[')?;
    let bytes = combined.as_bytes();
    let mut depth = 0i32;
    for (offset, byte) in bytes[start..].iter().enumerate() {
        match byte {
            b'[' => depth += 1,
            b']' => {
                depth -= 1;
                if depth == 0 {
                    let end = start + offset + 1;
                    return Some(combined[start..end].to_string());
                }
            }
            _ => {}
        }
    }
    None
}

/// Pulls the top-level key of the first diagnostic's `error` object out of a
/// weaver `--diagnostic-format json` array, e.g. `"Resolver"` from
/// `[{"error": {"Resolver": {...}}}]`. Uses `serde_json::Value` (already a
/// crate dependency) rather than a bespoke parser.
fn first_diagnostic_error_kind(diagnostics_json: &str) -> Option<String> {
    let value: serde_json::Value = serde_json::from_str(diagnostics_json).ok()?;
    let first = value.as_array()?.first()?;
    let error = first.get("error")?.as_object()?;
    error.keys().next().cloned()
}

/// Convenience: whether `weaver_bin` (a path or a name resolvable on `$PATH`)
/// looks runnable at all, by checking it exists as a file when it looks like a
/// path. Mirrors the crate's existing pattern of exposing a small runtime
/// "is the real collaborator available" probe (see `chicago-tdd-tools`'
/// `is_server_available`-style checks) so callers can skip gracefully instead
/// of failing with a confusing spawn error.
pub fn weaver_binary_exists(weaver_bin: &str) -> bool {
    if weaver_bin.contains('/') {
        Path::new(weaver_bin).is_file()
    } else {
        // A bare name: assume PATH resolution is weaver's own concern: a
        // definitive check would require walking $PATH ourselves, which is
        // more machinery than this structure-only probe needs. Callers using
        // a bare name should rely on `Command::output`'s own spawn error via
        // `WeaverRefusalReason::ProcessFailure` instead.
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::admission::Admit;
    use crate::witness::Witness;

    /// The real binary this session confirmed is the only `weaver` on this
    /// machine. No `weaver` on `$PATH`.
    const REAL_WEAVER_BIN: &str = "/Users/sac/chicago-tdd-tools/target/debug/weaver";

    fn write_registry(dir: &std::path::Path, manifest: &str, group: &str) {
        std::fs::create_dir_all(dir.join("model")).unwrap();
        std::fs::write(dir.join("registry_manifest.yaml"), manifest).unwrap();
        std::fs::write(dir.join("model").join("test.yaml"), group).unwrap();
    }

    /// A real, on-disk, valid-enough registry admits (exit 0), even though
    /// weaver still emits non-fatal warning diagnostics for it — proving
    /// `Admitted` does not require an empty diagnostics array, only success.
    #[test]
    fn admits_a_real_passing_registry() {
        if !weaver_binary_exists(REAL_WEAVER_BIN) {
            eprintln!("SKIP: real weaver binary not present at {REAL_WEAVER_BIN}");
            return;
        }
        let dir = std::env::temp_dir().join(format!(
            "wasm4pm-compat-weaver-admit-{}",
            std::process::id()
        ));
        write_registry(
            &dir,
            "name: test-registry\ndescription: minimal test registry\nschema_base_url: https://example.com/schemas\nsemconv_version: 0.1.0\n",
            "groups:\n  - id: test.group\n    type: attribute_group\n    brief: \"A minimal test attribute group.\"\n    attributes:\n      - id: test.attr\n        type: string\n        brief: \"A test attribute.\"\n        stability: development\n",
        );

        let raw = Evidence::<_, Raw, WeaverSemconv>::raw(WeaverRegistryInput::with_binary(
            dir.to_string_lossy().to_string(),
            REAL_WEAVER_BIN,
        ));
        let admission = WeaverCheckAdmit::admit(raw).expect("real registry should be admitted");
        assert_eq!(admission.value.input.registry, dir.to_string_lossy());

        std::fs::remove_dir_all(&dir).ok();
    }

    /// A real, on-disk registry with a dangling attribute reference is
    /// refused with the real named diagnostic kind weaver itself reports
    /// (`Resolver`), not a bare "InvalidInput".
    #[test]
    fn refuses_a_real_broken_registry_with_named_reason() {
        if !weaver_binary_exists(REAL_WEAVER_BIN) {
            eprintln!("SKIP: real weaver binary not present at {REAL_WEAVER_BIN}");
            return;
        }
        let dir = std::env::temp_dir().join(format!(
            "wasm4pm-compat-weaver-refuse-{}",
            std::process::id()
        ));
        write_registry(
            &dir,
            "name: bad-registry\ndescription: broken test registry\nschema_url: https://example.com/schemas/0.1.0\n",
            "groups:\n  - id: test.group\n    type: attribute_group\n    brief: \"A group referencing a nonexistent attribute.\"\n    attributes:\n      - ref: nonexistent.attr\n",
        );

        let raw = Evidence::<_, Raw, WeaverSemconv>::raw(WeaverRegistryInput::with_binary(
            dir.to_string_lossy().to_string(),
            REAL_WEAVER_BIN,
        ));
        let refusal = WeaverCheckAdmit::admit(raw).expect_err("broken registry must be refused");
        match refusal.into_reason() {
            WeaverRefusalReason::Named {
                kind,
                diagnostics_json,
            } => {
                assert_eq!(kind, "Resolver");
                assert!(diagnostics_json.contains("UnresolvedAttributeRef"));
            }
            other => panic!("expected a Named refusal reason, got {other:?}"),
        }

        std::fs::remove_dir_all(&dir).ok();
    }

    /// A nonexistent weaver binary produces a `ProcessFailure` refusal
    /// carrying the real spawn error, never a panic or a silent default.
    #[test]
    fn refuses_when_weaver_binary_does_not_exist() {
        let raw = Evidence::<_, Raw, WeaverSemconv>::raw(WeaverRegistryInput::with_binary(
            "/nonexistent/registry",
            "/nonexistent/path/to/weaver-binary-that-does-not-exist",
        ));
        let refusal = WeaverCheckAdmit::admit(raw).expect_err("missing binary must be refused");
        match refusal.into_reason() {
            WeaverRefusalReason::ProcessFailure { output, .. } => {
                assert!(!output.is_empty());
            }
            other => panic!("expected ProcessFailure, got {other:?}"),
        }
    }

    #[test]
    fn witness_metadata_is_real() {
        assert_eq!(WeaverSemconv::KEY, "weaver-semconv");
        assert_eq!(WeaverSemconv::FAMILY, WitnessFamily::Standard);
    }
}
