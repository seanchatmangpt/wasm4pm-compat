//! Real cross-language subprocess bridge to `wasm4pm_compat::ocel_diff`.
//!
//! Run with: `cargo run --example ocel_diff_cli -- <before.json> <after.json> <expected.json>`
//!
//! Each argument is a path to a JSON file shaped `{"facts": {...}}`
//! (`ocel_diff::StateSnapshot`'s real serde shape). Prints, on stdout, the
//! real `(StateDiff, EffectMatchResult)` pair as one JSON object
//! `{"diff": ..., "match_result": ...}` -- so a caller in another language
//! (autofde-lab's Python integration suite) can invoke this exact binary
//! via `subprocess`, feed it the platform-console route's real before/after
//! JSON snapshots plus a declared expected-effect snapshot, and parse the
//! real Rust diff/match result back out, the same external-process pattern
//! `castle.ts` already uses for its own subprocess calls elsewhere in this
//! ecosystem. No behavior is duplicated in Rust here -- this is a thin CLI
//! shell around the real `ocel_diff::diff_and_match` function exercised by
//! `src/ocel_diff.rs`'s own unit tests.

use std::fs;
use std::process::ExitCode;

use wasm4pm_compat::ocel_diff::{diff_and_match, StateSnapshot};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 3 {
        eprintln!("usage: ocel_diff_cli <before.json> <after.json> <expected_effect.json>");
        return ExitCode::from(64);
    }

    let load = |path: &str| -> Result<StateSnapshot, String> {
        let text = fs::read_to_string(path).map_err(|e| format!("read {path}: {e}"))?;
        serde_json::from_str(&text).map_err(|e| format!("parse {path}: {e}"))
    };

    let before = match load(&args[0]) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("ocel_diff_cli: {e}");
            return ExitCode::from(1);
        }
    };
    let after = match load(&args[1]) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("ocel_diff_cli: {e}");
            return ExitCode::from(1);
        }
    };
    let expected = match load(&args[2]) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("ocel_diff_cli: {e}");
            return ExitCode::from(1);
        }
    };

    let (diff, match_result) = diff_and_match(&before, &after, &expected);
    let output = serde_json::json!({
        "diff": diff,
        "match_result": match_result,
    });
    println!(
        "{}",
        serde_json::to_string(&output).expect("serialize output")
    );

    if match_result.matches {
        ExitCode::from(0)
    } else {
        ExitCode::from(2)
    }
}
