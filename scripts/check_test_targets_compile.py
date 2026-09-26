#!/usr/bin/env python3
"""Test-target compile-closure guard.

Law: every integration test file ``tests/*.rs`` is a registered, compiling test
target. CI runs ``cargo test --locked --all-features --tests``; one target that
fails to compile turns the whole crown red while hiding which files are broken
behind the first error. This guard compiles every test target with
``--no-fail-fast`` (keep going past the first failure), reads cargo's JSON
message stream, and refuses with the exact list of targets that did not produce
a test artifact, plus the first compiler error of each.

Origin: v26.9.26 frontier item wasm4pm-compat-test-target-compile-closure
(ws4_protocol_91..96 referenced ``Intent``, which the prelude only re-exports
as ProtocolIntent/DoctorIntent, and court 96 imported the nonexistent path
``wasm4pm_compat::protocol``; 181 of 187 targets compiled).

Exit codes: 0 = closure holds; 1 = REFUSED(TEST_TARGET_COMPILE_CLOSURE);
2 = cargo could not be run or produced no parseable stream.

Authority: NONE. Ceiling: SELECT (read-only observation + cargo build cache).
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path


def expected_targets(tests_dir: Path) -> dict[str, str]:
    """Resolved source path -> stem of top-level ``tests/*.rs`` files.

    Keyed by source path, not by target name: an explicit ``[[test]]`` entry may
    register ``tests/orig.rs`` under another name (``name = "renamed"``), and a
    ``[[test]]`` whose *name* equals a file stem may point at a different file
    while that file stays unregistered. Matching by name gives a false refusal
    in the first case and a false admission in the second.
    """
    return {str(p.resolve()): p.stem for p in tests_dir.glob("*.rs") if p.is_file()}


def _resolve_keys(by_src: dict[str, str]) -> dict[str, str]:
    """Resolve src_path keys once per distinct test target (not per JSON line)."""
    return {str(Path(src).resolve()): value for src, value in by_src.items()}


def parse_stream(lines):
    """Return (compiled, errors, upstream) from a cargo JSON message stream.

    compiled: raw src_path -> target name for every test target that
    produced a test-profile artifact. errors: resolved src_path -> first error of
    that test target. upstream: "<kind>:<name>" -> first error of a non-test
    target (e.g. the library), which explains why every test is missing.
    Non-JSON lines, malformed JSON, non-dict payloads and duplicated or
    reordered messages are tolerated (sets/first-wins), never crash the guard.
    """
    compiled: dict[str, str] = {}
    errors: dict[str, str] = {}
    upstream: dict[str, str] = {}
    for raw in lines:
        raw = raw.strip()
        if not raw.startswith("{"):
            continue
        try:
            msg = json.loads(raw)
        except json.JSONDecodeError:
            continue
        if not isinstance(msg, dict):
            continue
        target = msg.get("target")
        if not isinstance(target, dict):
            continue
        kinds = target.get("kind") or []
        name = target.get("name")
        key = target.get("src_path")
        if not isinstance(name, str) or not name or not isinstance(key, str) or not key:
            continue
        reason = msg.get("reason")
        inner = msg.get("message") if reason == "compiler-message" else None
        first_error = None
        if isinstance(inner, dict) and inner.get("level") == "error":
            code = (inner.get("code") or {}).get("code") or ""
            first_error = f"{code} {inner.get('message') or ''}".strip()
        if "test" not in kinds:
            if first_error is not None:
                upstream.setdefault(f"{'/'.join(kinds)}:{name}", first_error)
            continue
        profile = msg.get("profile") or {}
        if reason == "compiler-artifact" and profile.get("test"):
            compiled[key] = name
        elif first_error is not None:
            errors.setdefault(key, first_error)
    return compiled, errors, upstream


def run(manifest_path: Path, cargo_args: list[str]) -> int:
    root = manifest_path.parent
    expected = expected_targets(root / "tests")
    if not expected:
        print(f"REFUSED(NO_TEST_TARGETS): no tests/*.rs under {root}", file=sys.stderr)
        return 1
    cmd = [
        "cargo",
        "test",
        "--manifest-path",
        str(manifest_path),
        "--tests",
        "--no-run",
        "--no-fail-fast",
        "--message-format=json",
        *cargo_args,
    ]
    proc = subprocess.run(cmd, cwd=root, capture_output=True, text=True)
    compiled, errors, upstream = parse_stream(proc.stdout.splitlines())
    compiled, errors = _resolve_keys(compiled), _resolve_keys(errors)
    if not compiled and not errors and not upstream:
        sys.stderr.write(proc.stderr[-4000:])
        print(f"BLOCKED(CARGO_NO_STREAM): exit={proc.returncode} cmd={' '.join(cmd)}", file=sys.stderr)
        return 2
    missing = sorted(stem for path, stem in expected.items() if path not in compiled)
    print(
        json.dumps(
            {
                "law": "test-target-compile-closure",
                "expected": len(expected),
                "compiled": len(expected.keys() & compiled.keys()),
                "missing": missing,
                "upstream_errors": sorted(upstream),
                "cargo_exit": proc.returncode,
            },
            sort_keys=True,
        )
    )
    if missing or proc.returncode != 0:
        by_stem = {expected[p]: e for p, e in errors.items() if p in expected}
        fallback = "no test artifact produced"
        if upstream:
            first = sorted(upstream)[0]
            fallback = f"blocked upstream by {first}: {upstream[first]}"
        for name in missing:
            print(f"  FAIL {name}: {by_stem.get(name, fallback)}", file=sys.stderr)
        if not missing:
            sys.stderr.write(proc.stderr[-4000:])
        print(
            f"REFUSED(TEST_TARGET_COMPILE_CLOSURE): {len(missing)} of {len(expected)} "
            "test targets did not compile",
            file=sys.stderr,
        )
        return 1
    return 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--manifest-path", type=Path, default=Path("Cargo.toml"))
    parser.add_argument("cargo_args", nargs="*", help="extra cargo args, e.g. --locked --all-features")
    args = parser.parse_args(argv)
    return run(args.manifest_path.resolve(), args.cargo_args)


if __name__ == "__main__":
    sys.exit(main())
