#!/usr/bin/env python3
"""Deterministic benchmark for scripts/check_test_targets_compile.py.

Two measurements, both on real code paths (no doubles):
  1. parse_stream throughput over synthetic cargo JSON streams shaped like a
     real ``cargo test --no-run --message-format=json`` run (one dependency
     artifact per test target plus one test artifact), sizes 187 (this crate),
     2k and 20k targets; median of --reps runs.
  2. optional (--guard): wall time of the real guard on this repository with a
     warm build cache (``python3 scripts/check_test_targets_compile.py -- ...``).

Prints one JSON object; --out writes it as a receipt. The regression bound is
enforced as a unit test (test_parse_throughput_regression_bound) and re-checked
here: exit 1 if the 20k-target median exceeds BOUND_20K_SECONDS.

Authority: NONE. Ceiling: SELECT.
"""

from __future__ import annotations

import argparse
import importlib.util
import json
import platform
import statistics
import subprocess
import sys
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
GUARD = HERE / "check_test_targets_compile.py"
BOUND_20K_SECONDS = 2.0

_SPEC = importlib.util.spec_from_file_location("check_test_targets_compile", GUARD)
guard = importlib.util.module_from_spec(_SPEC)
_SPEC.loader.exec_module(guard)


def synthetic_stream(n: int) -> list[str]:
    lines = []
    for i in range(n):
        lines.append(
            json.dumps(
                {
                    "reason": "compiler-artifact",
                    "target": {"kind": ["lib"], "name": f"dep{i}", "src_path": f"/r/deps/{i}/lib.rs"},
                    "profile": {"test": False},
                }
            )
        )
        lines.append(
            json.dumps(
                {
                    "reason": "compiler-artifact",
                    "target": {"kind": ["test"], "name": f"t{i}", "src_path": f"/r/tests/t{i}.rs"},
                    "profile": {"test": True},
                }
            )
        )
    return lines


def time_parse(n: int, reps: int) -> dict:
    lines = synthetic_stream(n)
    samples = []
    for _ in range(reps):
        start = time.perf_counter()
        compiled, _, _ = guard.parse_stream(lines)
        samples.append(time.perf_counter() - start)
        assert len(compiled) == n, (len(compiled), n)
    med = statistics.median(samples)
    return {
        "targets": n,
        "lines": len(lines),
        "median_s": round(med, 6),
        "min_s": round(min(samples), 6),
        "max_s": round(max(samples), 6),
        "lines_per_s": int(len(lines) / med) if med else None,
    }


def time_guard(root: Path, reps: int) -> dict:
    rel = str(GUARD.relative_to(root)) if GUARD.is_relative_to(root) else str(GUARD)
    cmd = [sys.executable, rel, "--", "--locked", "--all-features"]
    samples, last = [], None
    for _ in range(reps):
        start = time.perf_counter()
        proc = subprocess.run(cmd, cwd=root, capture_output=True, text=True)
        samples.append(time.perf_counter() - start)
        last = proc
    return {
        "cmd": " ".join(cmd[1:]),
        "exit": last.returncode,
        "verdict": last.stdout.strip().splitlines()[-1] if last.stdout.strip() else "",
        "median_s": round(statistics.median(samples), 3),
        "samples_s": [round(s, 3) for s in samples],
    }


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--reps", type=int, default=7)
    ap.add_argument("--guard", action="store_true", help="also time the real guard on this repo (warm cache)")
    ap.add_argument("--guard-reps", type=int, default=3)
    ap.add_argument("--out", type=Path)
    args = ap.parse_args(argv)
    report = {
        "bench": "check_test_targets_compile",
        "python": platform.python_version(),
        "machine": platform.machine(),
        "system": platform.system(),
        "bound_20k_targets_s": BOUND_20K_SECONDS,
        "parse_stream": [time_parse(n, args.reps) for n in (187, 2_000, 20_000)],
    }
    if args.guard:
        report["guard_warm"] = time_guard(HERE.parent, args.guard_reps)
    worst = report["parse_stream"][-1]["median_s"]
    report["within_bound"] = worst < BOUND_20K_SECONDS
    text = json.dumps(report, indent=2, sort_keys=True)
    print(text)
    if args.out:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text + "\n")
    return 0 if report["within_bound"] else 1


if __name__ == "__main__":
    sys.exit(main())
