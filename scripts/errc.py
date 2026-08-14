#!/usr/bin/env python3
"""ERRC v2: deterministic evidence-preserving reduction and replay.

Reconstituted from the routing, receipt, replay, and falsification mechanisms in
seanchatmangpt/ggen-legacy@60d38265b8d1d94c43f04ca6bdb8537184e510a8.
This is verification/manufacturing tooling, not runtime process-mining logic.
"""
from __future__ import annotations

import argparse
import hashlib
import itertools
import json
import re
import sys
from dataclasses import dataclass
from enum import Enum
from pathlib import Path
from typing import Any, Mapping, Sequence

SCHEMA_MATRIX = "errc.matrix.v2"
SCHEMA_RECEIPT = "errc.receipt.v2"
HEAD_RE = re.compile(r"^[0-9a-f]{40}$")


class Observation(str, Enum):
    KILL = "KILL"
    SURVIVE = "SURVIVE"
    UNKNOWN = "UNKNOWN"


class RefusalCode(str, Enum):
    INVALID_MATRIX = "INVALID_MATRIX"
    UNKNOWN_OBSERVATION = "UNKNOWN_OBSERVATION"
    INVALID_HEAD = "INVALID_HEAD"
    SOLVER_LIMIT_EXCEEDED = "SOLVER_LIMIT_EXCEEDED"
    RECEIPT_MISMATCH = "RECEIPT_MISMATCH"
    WITNESS_MISSING = "WITNESS_MISSING"


class ERRCRefusal(Exception):
    def __init__(self, code: RefusalCode, message: str):
        super().__init__(message)
        self.code = code
        self.message = message

    def as_dict(self) -> dict[str, str]:
        return {"status": "REFUSED", "code": self.code.value, "message": self.message}


@dataclass(frozen=True)
class Matrix:
    tests: tuple[str, ...]
    falsifiers: tuple[str, ...]
    cells: tuple[tuple[Observation, ...], ...]

    def state(self, ti: int, mi: int) -> Observation:
        return self.cells[ti][mi]

    def canonical_dict(self) -> dict[str, Any]:
        return {
            "schema": SCHEMA_MATRIX,
            "tests": list(self.tests),
            "falsifiers": list(self.falsifiers),
            "observations": {
                test: {
                    falsifier: self.cells[ti][mi].value
                    for mi, falsifier in enumerate(self.falsifiers)
                }
                for ti, test in enumerate(self.tests)
            },
        }


def canonical_json(value: Any) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def digest(value: Any) -> str:
    return hashlib.sha256(canonical_json(value).encode()).hexdigest()


def _ids(value: Any, label: str) -> tuple[str, ...]:
    if not isinstance(value, list):
        raise ERRCRefusal(RefusalCode.INVALID_MATRIX, f"{label} must be an array")
    if any(not isinstance(x, str) or not x.strip() for x in value):
        raise ERRCRefusal(RefusalCode.INVALID_MATRIX, f"{label} IDs must be non-empty strings")
    result = tuple(value)
    if len(set(result)) != len(result):
        raise ERRCRefusal(RefusalCode.INVALID_MATRIX, f"{label} IDs must be unique")
    if tuple(sorted(result)) != result:
        raise ERRCRefusal(RefusalCode.INVALID_MATRIX, f"{label} IDs must be lexicographically sorted")
    return result


def parse_matrix(document: Mapping[str, Any]) -> Matrix:
    if not isinstance(document, Mapping):
        raise ERRCRefusal(RefusalCode.INVALID_MATRIX, "matrix must be an object")
    if document.get("schema", SCHEMA_MATRIX) != SCHEMA_MATRIX:
        raise ERRCRefusal(RefusalCode.INVALID_MATRIX, "unsupported matrix schema")
    tests = _ids(document.get("tests"), "tests")
    falsifiers = _ids(document.get("falsifiers"), "falsifiers")
    observations = document.get("observations")
    if not isinstance(observations, Mapping):
        raise ERRCRefusal(RefusalCode.INVALID_MATRIX, "observations must be an object")
    if set(observations) != set(tests):
        raise ERRCRefusal(RefusalCode.INVALID_MATRIX, "observation rows must exactly match tests")

    rows: list[tuple[Observation, ...]] = []
    unknown: list[str] = []
    for test in tests:
        row = observations[test]
        if not isinstance(row, Mapping) or set(row) != set(falsifiers):
            raise ERRCRefusal(RefusalCode.INVALID_MATRIX, f"row {test!r} is not rectangular")
        parsed: list[Observation] = []
        for falsifier in falsifiers:
            try:
                state = Observation(row[falsifier])
            except (ValueError, TypeError):
                raise ERRCRefusal(
                    RefusalCode.INVALID_MATRIX,
                    f"invalid observation at {test}/{falsifier}: {row[falsifier]!r}",
                ) from None
            if state is Observation.UNKNOWN:
                unknown.append(f"{test}/{falsifier}")
            parsed.append(state)
        rows.append(tuple(parsed))
    if unknown:
        sample = ", ".join(unknown[:8])
        raise ERRCRefusal(
            RefusalCode.UNKNOWN_OBSERVATION,
            f"exact compression requires closed evidence; UNKNOWN at {sample}",
        )
    return Matrix(tests, falsifiers, tuple(rows))


def validate_subject(repository: str, head: str) -> None:
    if not isinstance(repository, str) or repository.count("/") != 1:
        raise ERRCRefusal(RefusalCode.INVALID_HEAD, "repository must be owner/name")
    if not isinstance(head, str) or not HEAD_RE.fullmatch(head):
        raise ERRCRefusal(RefusalCode.INVALID_HEAD, "head must be an exact lowercase 40-hex SHA")


def obligation_universe(matrix: Matrix) -> tuple[str, ...]:
    obligations: set[str] = set()
    for mi, falsifier in enumerate(matrix.falsifiers):
        if any(matrix.state(ti, mi) is Observation.KILL for ti in range(len(matrix.tests))):
            obligations.add(f"kill:{falsifier}")
    for li, ri in itertools.combinations(range(len(matrix.falsifiers)), 2):
        if any(matrix.state(ti, li) is not matrix.state(ti, ri) for ti in range(len(matrix.tests))):
            obligations.add(f"dist:{matrix.falsifiers[li]}|{matrix.falsifiers[ri]}")
    return tuple(sorted(obligations))


def coverage_by_test(matrix: Matrix, obligations: Sequence[str]) -> dict[str, frozenset[str]]:
    universe = set(obligations)
    index = {name: i for i, name in enumerate(matrix.falsifiers)}
    result: dict[str, frozenset[str]] = {}
    for ti, test in enumerate(matrix.tests):
        covered: set[str] = set()
        for falsifier, mi in index.items():
            key = f"kill:{falsifier}"
            if key in universe and matrix.state(ti, mi) is Observation.KILL:
                covered.add(key)
        for left, right in itertools.combinations(matrix.falsifiers, 2):
            key = f"dist:{left}|{right}"
            if key in universe and matrix.state(ti, index[left]) is not matrix.state(ti, index[right]):
                covered.add(key)
        result[test] = frozenset(covered)
    return result


def _first_cover_of_size(
    tests: Sequence[str],
    coverage: Mapping[str, frozenset[str]],
    universe: frozenset[str],
    target: int,
) -> tuple[str, ...] | None:
    """Exact branch-and-bound; first hit is canonical lexicographic minimum."""
    n = len(tests)
    suffix: list[frozenset[str]] = [frozenset() for _ in range(n + 1)]
    for i in range(n - 1, -1, -1):
        suffix[i] = suffix[i + 1] | coverage[tests[i]]

    def visit(start: int, chosen: tuple[str, ...], covered: frozenset[str]) -> tuple[str, ...] | None:
        slots = target - len(chosen)
        if slots == 0:
            return chosen if universe <= covered else None
        if n - start < slots or not (universe - covered) <= suffix[start]:
            return None
        for i in range(start, n - slots + 1):
            found = visit(i + 1, chosen + (tests[i],), covered | coverage[tests[i]])
            if found is not None:
                return found
        return None

    return visit(0, tuple(), frozenset())


def exact_minimum_cover(
    matrix: Matrix,
    max_tests: int = 22,
    max_obligations: int = 4096,
) -> tuple[tuple[str, ...], tuple[str, ...], dict[str, frozenset[str]]]:
    if len(matrix.tests) > max_tests:
        raise ERRCRefusal(
            RefusalCode.SOLVER_LIMIT_EXCEEDED,
            f"exact solver admits <= {max_tests} tests; observed {len(matrix.tests)}",
        )
    obligations = obligation_universe(matrix)
    if len(obligations) > max_obligations:
        raise ERRCRefusal(
            RefusalCode.SOLVER_LIMIT_EXCEEDED,
            f"exact solver admits <= {max_obligations} obligations; observed {len(obligations)}",
        )
    coverage = coverage_by_test(matrix, obligations)
    universe = frozenset(obligations)
    if not universe:
        return tuple(), obligations, coverage
    candidates = tuple(t for t in matrix.tests if coverage[t])
    max_gain = max((len(coverage[t]) for t in candidates), default=0)
    if max_gain == 0:
        raise ERRCRefusal(RefusalCode.WITNESS_MISSING, "obligations have no evidence witness")
    lower = (len(universe) + max_gain - 1) // max_gain
    for size in range(lower, len(candidates) + 1):
        retained = _first_cover_of_size(candidates, coverage, universe, size)
        if retained is not None:
            return retained, obligations, coverage
    raise ERRCRefusal(RefusalCode.WITNESS_MISSING, "no subset preserves all obligations")


def structural_analysis(matrix: Matrix, coverage: Mapping[str, frozenset[str]]) -> dict[str, Any]:
    signatures: dict[tuple[str, ...], list[str]] = {}
    for test in matrix.tests:
        signatures.setdefault(tuple(sorted(coverage[test])), []).append(test)
    equivalent = [members for _, members in sorted(signatures.items()) if len(members) > 1]
    dominance = sorted(
        [left, right]
        for left, right in itertools.permutations(matrix.tests, 2)
        if coverage[left] and coverage[left] < coverage[right]
    )
    return {"equivalent_tests": equivalent, "strict_dominance": dominance}


def witnesses(
    retained: Sequence[str], obligations: Sequence[str], coverage: Mapping[str, frozenset[str]]
) -> dict[str, str]:
    result: dict[str, str] = {}
    for obligation in obligations:
        witness = next((test for test in retained if obligation in coverage[test]), None)
        if witness is None:
            raise ERRCRefusal(RefusalCode.WITNESS_MISSING, f"missing witness for {obligation}")
        result[obligation] = witness
    return result


def build_receipt(
    document: Mapping[str, Any],
    repository: str,
    head: str,
    max_tests: int = 22,
    max_obligations: int = 4096,
) -> dict[str, Any]:
    validate_subject(repository, head)
    matrix = parse_matrix(document)
    retained, obligations, coverage = exact_minimum_cover(matrix, max_tests, max_obligations)
    proof_witnesses = witnesses(retained, obligations, coverage)
    receipt: dict[str, Any] = {
        "schema": SCHEMA_RECEIPT,
        "subject": {"repository": repository, "head": head},
        "standing": "PARTIAL_ALIVE",
        "claim": "exact-evidence-preserving-compression-candidate",
        "matrix_digest": digest(matrix.canonical_dict()),
        "tests_digest": digest(list(matrix.tests)),
        "falsifiers_digest": digest(list(matrix.falsifiers)),
        "obligations_digest": digest(list(obligations)),
        "retained_digest": digest(list(retained)),
        "witnesses_digest": digest(proof_witnesses),
        "solver": {
            "algorithm": "exact-bounded-lexicographic-branch-and-bound",
            "max_tests": max_tests,
            "max_obligations": max_obligations,
            "tie_break": "lexicographic-test-id",
        },
        "counts": {
            "tests_original": len(matrix.tests),
            "tests_retained": len(retained),
            "falsifiers": len(matrix.falsifiers),
            "obligations": len(obligations),
        },
        "retained_tests": list(retained),
        "obligations": list(obligations),
        "witnesses": proof_witnesses,
        "structure": structural_analysis(matrix, coverage),
        "provenance": {
            "reconstituted_from": "seanchatmangpt/ggen-legacy",
            "source_head": "60d38265b8d1d94c43f04ca6bdb8537184e510a8",
            "laws": [
                "kill-preservation", "diagnostic-preservation", "unknown-closure",
                "canonical-minimality", "witness-completeness", "exact-subject-binding",
                "independent-replay", "tamper-refusal",
            ],
        },
    }
    receipt["receipt_digest"] = digest(receipt)
    return receipt


def verify_receipt(document: Mapping[str, Any], receipt: Mapping[str, Any]) -> dict[str, Any]:
    if not isinstance(receipt, Mapping) or receipt.get("schema") != SCHEMA_RECEIPT:
        raise ERRCRefusal(RefusalCode.RECEIPT_MISMATCH, "unsupported receipt schema")
    supplied = receipt.get("receipt_digest")
    unsigned = dict(receipt)
    unsigned.pop("receipt_digest", None)
    if supplied != digest(unsigned):
        raise ERRCRefusal(RefusalCode.RECEIPT_MISMATCH, "receipt digest mismatch")
    try:
        subject, solver = receipt["subject"], receipt["solver"]
        expected = build_receipt(
            document,
            str(subject["repository"]),
            str(subject["head"]),
            int(solver["max_tests"]),
            int(solver["max_obligations"]),
        )
    except (KeyError, TypeError, ValueError):
        raise ERRCRefusal(RefusalCode.RECEIPT_MISMATCH, "malformed receipt") from None
    if canonical_json(expected) != canonical_json(dict(receipt)):
        raise ERRCRefusal(
            RefusalCode.RECEIPT_MISMATCH,
            "independent replay does not reproduce supplied receipt",
        )
    return {
        "status": "VERIFIED",
        "claim": "exact-evidence-preserving-compression-replayed",
        "subject": expected["subject"],
        "receipt_digest": expected["receipt_digest"],
        "tests_original": expected["counts"]["tests_original"],
        "tests_retained": expected["counts"]["tests_retained"],
        "obligations": expected["counts"]["obligations"],
    }


def _load(path: str) -> Any:
    with open(path, encoding="utf-8") as handle:
        return json.load(handle)


def _write(path: str | None, value: Any) -> None:
    text = json.dumps(value, sort_keys=True, indent=2) + "\n"
    if path:
        Path(path).write_text(text, encoding="utf-8")
    else:
        sys.stdout.write(text)


def parser() -> argparse.ArgumentParser:
    root = argparse.ArgumentParser(description="ERRC v2 evidence-preserving minimizer")
    commands = root.add_subparsers(dest="command", required=True)
    compress = commands.add_parser("compress")
    compress.add_argument("--input", required=True)
    compress.add_argument("--repo", required=True)
    compress.add_argument("--head", required=True)
    compress.add_argument("--output")
    compress.add_argument("--max-tests", type=int, default=22)
    compress.add_argument("--max-obligations", type=int, default=4096)
    verify = commands.add_parser("verify")
    verify.add_argument("--input", required=True)
    verify.add_argument("--receipt", required=True)
    verify.add_argument("--output")
    return root


def main(argv: Sequence[str] | None = None) -> int:
    args = parser().parse_args(argv)
    try:
        matrix = _load(args.input)
        if args.command == "compress":
            result = build_receipt(matrix, args.repo, args.head, args.max_tests, args.max_obligations)
            _write(args.output, result)
        else:
            _write(args.output, verify_receipt(matrix, _load(args.receipt)))
        return 0
    except ERRCRefusal as refusal:
        sys.stderr.write(canonical_json(refusal.as_dict()) + "\n")
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
