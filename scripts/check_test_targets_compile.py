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


def expected_targets(tests_dir: Path) -> set[str]:
    """Stems of top-level ``tests/*.rs`` files (cargo autotests convention)."""
    return {p.stem for p in tests_dir.glob("*.rs") if p.is_file()}


def parse_stream(lines):
    """Return (compiled_test_targets, first_error_by_target) from cargo JSON."""
    compiled: set[str] = set()
    errors: dict[str, str] = {}
    for raw in lines:
        raw = raw.strip()
        if not raw.startswith("{"):
            continue
        try:
            msg = json.loads(raw)
        except json.JSONDecodeError:
            continue
        target = msg.get("target") or {}
        if "test" not in (target.get("kind") or []):
            continue
        name = target.get("name")
        if not name:
            continue
        reason = msg.get("reason")
        if reason == "compiler-artifact" and (msg.get("profile") or {}).get("test"):
            compiled.add(name)
        elif reason == "compiler-message":
            inner = msg.get("message") or {}
            if inner.get("level") == "error" and name not in errors:
                code = (inner.get("code") or {}).get("code") or ""
                text = inner.get("message") or ""
                errors[name] = f"{code} {text}".strip()
    return compiled, errors


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
    compiled, errors = parse_stream(proc.stdout.splitlines())
    if not compiled and not errors:
        sys.stderr.write(proc.stderr[-4000:])
        print(f"BLOCKED(CARGO_NO_STREAM): exit={proc.returncode} cmd={' '.join(cmd)}", file=sys.stderr)
        return 2
    missing = sorted(expected - compiled)
    print(
        json.dumps(
            {
                "law": "test-target-compile-closure",
                "expected": len(expected),
                "compiled": len(compiled & expected),
                "missing": missing,
                "cargo_exit": proc.returncode,
            },
            sort_keys=True,
        )
    )
    if missing or proc.returncode != 0:
        for name in missing:
            print(f"  FAIL {name}: {errors.get(name, 'no test artifact produced')}", file=sys.stderr)
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
