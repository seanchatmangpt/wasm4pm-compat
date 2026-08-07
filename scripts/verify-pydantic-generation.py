#!/usr/bin/env python3
"""Verify Rust-source → ontology → ggen → Pydantic closure.

Graph closure and literal Rust source coverage are reported separately. The
receipt cannot be ALIVE while the exact Rust source tree is unavailable or a
public Serde data type lacks an admitted generated projection.
"""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import re
import sys
from pathlib import Path
from typing import Any

from rdflib import Graph, Namespace

COMPAT = Namespace("https://wasm4pm-compat.rs/ontology#")
SIMPLE_RUST_TYPE = re.compile(r"^[A-Za-z_][A-Za-z0-9_]*$")
SERDE_PUBLIC_TYPE = re.compile(
    r"#\s*\[\s*derive\s*\((?P<derives>[^)]*(?:Serialize|Deserialize)[^)]*)\)\s*\]"
    r"(?:\s*#\s*\[[^\]]*\])*\s*pub\s+(?:struct|enum)\s+(?P<name>[A-Za-z_][A-Za-z0-9_]*)",
    re.MULTILINE | re.DOTALL,
)


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def class_name(subject: Any, rust_type: str) -> str:
    if SIMPLE_RUST_TYPE.fullmatch(rust_type):
        return rust_type
    return re.split(r"[#/]", str(subject))[-1]


def load_module(path: Path):
    spec = importlib.util.spec_from_file_location("wasm4pm_generated_verifier", path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load generated module: {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def scan_source(source_root: Path) -> tuple[str, list[str], dict[str, list[str]]]:
    files = sorted(source_root.rglob("*.rs")) if source_root.is_dir() else []
    if not files:
        return "UNKNOWN", [], {}
    observed: dict[str, list[str]] = {}
    for path in files:
        text = path.read_text(encoding="utf-8", errors="replace")
        for match in SERDE_PUBLIC_TYPE.finditer(text):
            observed.setdefault(match.group("name"), []).append(str(path))
    return "OBSERVED", sorted(observed), observed


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument("--source-root", type=Path)
    parser.add_argument("--receipt", type=Path)
    args = parser.parse_args()
    root = args.root.resolve()
    source_root = (args.source_root or root / "src").resolve()
    ontology_paths = [
        root / "ggen/ontology/wasm4pm-compat.ttl",
        root / "ggen/ontology/zod-types.ttl",
    ]
    query_path = root / "ggen/queries/extract-pydantic-models.rq"
    template_path = root / "ggen/templates/pydantic-models.py.tera"
    manifest_path = root / "python/ggen.toml"
    generated_path = root / "python/src/wasm4pm_compat_pydantic/generated.py"

    graph = Graph()
    for path in ontology_paths:
        graph.parse(path, format="turtle")

    candidates: dict[str, list[tuple[str, bool]]] = {}
    zod = Namespace("https://wasm4pm-compat.rs/zod#")
    for subject, rust_type_node in graph.subject_objects(COMPAT.rustType):
        rust_type = str(rust_type_node)
        name = class_name(subject, rust_type)
        shaped = any(True for _ in graph.objects(subject, zod.hasField))
        candidates.setdefault(name, []).append((rust_type, shaped))

    expected: dict[str, str] = {}
    duplicate_conflicts: dict[str, list[str]] = {}
    shadowed_structural_types: dict[str, list[str]] = {}
    for name, entries in candidates.items():
        shaped = sorted({rust for rust, has_fields in entries if has_fields})
        structural = sorted({rust for rust, has_fields in entries if not has_fields})
        admitted = shaped if shaped else structural
        if len(admitted) != 1:
            duplicate_conflicts[name] = admitted
            continue
        expected[name] = admitted[0]
        shadowed = sorted(set(structural) - {admitted[0]}) if shaped else []
        if shadowed:
            shadowed_structural_types[name] = shadowed

    module = load_module(generated_path)
    actual = {
        name: getattr(module, name).__rust_type__
        for name in module.__all__
        if name not in {"CompatModel", "JsonNumber"}
    }
    missing = sorted(set(expected) - set(actual))
    extra = sorted(set(actual) - set(expected))
    rust_type_mismatches = {
        name: {"expected": expected[name], "actual": actual[name]}
        for name in sorted(set(expected) & set(actual))
        if expected[name] != actual[name]
    }

    schema_errors: dict[str, str] = {}
    for name in sorted(actual):
        try:
            getattr(module, name).model_json_schema()
        except Exception as exc:  # pragma: no cover - receipt captures exact failure
            schema_errors[name] = f"{type(exc).__name__}: {exc}"

    graph_failures = (duplicate_conflicts, missing, extra, rust_type_mismatches, schema_errors)
    graph_status = "ALIVE" if not any(graph_failures) else "PARTIAL_ALIVE"

    scan_status, serde_types, serde_locations = scan_source(source_root)
    source_missing = sorted(set(serde_types) - set(actual))
    if scan_status == "UNKNOWN":
        source_coverage_status = "UNKNOWN"
    else:
        source_coverage_status = "ALIVE" if not source_missing else "PARTIAL_ALIVE"

    status = "ALIVE" if graph_status == source_coverage_status == "ALIVE" else "PARTIAL_ALIVE"
    receipt = {
        "schema": "wasm4pm-compat.pydantic-generation-receipt.v2",
        "status": status,
        "graph_status": graph_status,
        "source_coverage_status": source_coverage_status,
        "source_root": str(source_root),
        "ontology_type_count": len(expected),
        "generated_type_count": len(actual),
        "public_serde_type_count": len(serde_types),
        "source_missing": source_missing,
        "source_locations": {name: serde_locations[name] for name in source_missing},
        "duplicate_conflicts": duplicate_conflicts,
        "shadowed_structural_types": shadowed_structural_types,
        "missing": missing,
        "extra": extra,
        "rust_type_mismatches": rust_type_mismatches,
        "schema_errors": schema_errors,
        "sha256": {
            str(path.relative_to(root)): digest(path)
            for path in [*ontology_paths, query_path, template_path, manifest_path, generated_path]
        },
    }
    rendered = json.dumps(receipt, indent=2, sort_keys=True) + "\n"
    print(rendered, end="")
    if args.receipt:
        args.receipt.parent.mkdir(parents=True, exist_ok=True)
        args.receipt.write_text(rendered)
    return 0 if status == "ALIVE" else 1


if __name__ == "__main__":
    raise SystemExit(main())
