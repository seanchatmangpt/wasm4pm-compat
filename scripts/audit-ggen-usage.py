#!/usr/bin/env python3
"""Fail-closed repository audit for the ggen 26.7.62 consumer contract."""

from __future__ import annotations

import argparse
import json
import re
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
GGEN_VERSION = "26.7.62"
GGEN_COMMIT = "68952593c40214ac1a681073d65f3902a9cdfce4"
SHADOW_MANIFESTS = (
    "ggen-witness.toml",
    "ggen/ggen.toml",
    "ggen/ggen-breed-scaffold.toml",
    "ggen/standing.ggen.toml",
    "ggen/package.toml",
)
ACTIVE_TEXT = (
    "AGENTS.md",
    "ggen.toml",
    "ggen/README.md",
    "ggen/Makefile.toml",
    "ggen/validate_ggen.sh",
    "scripts/verify-ggen-contract.sh",
    ".github/workflows/ci-control-plane.yml",
    "docs/architecture/GGEN_V26_7_31_RETROFIT.md",
    "docs/architecture/GGEN_26_7_62_CONVERGENCE.md",
)
FORBIDDEN_ACTIVE_PATTERNS = {
    "GGEN-PORTABILITY-001": re.compile(r"/Users/"),
    "GGEN-CROSS-REPO-001": re.compile(r"(?:\.\./)+wasm4pm|/wasm4pm/"),
    "GGEN-CLI-001": re.compile(r"ggen\s+generate\b"),
    "GGEN-CLI-002": re.compile(r"ggen\s+sync\s+--(?:manifest|locked|rule|queries|ontology|output-dir)\b"),
    "GGEN-CLI-003": re.compile(r"ggen\s+validate\b"),
    "GGEN-SOURCE-CASTE-001": re.compile(r"src/generated(?:/|\\b)"),
}


def check(condition: bool, code: str, message: str, checks: list[dict], failures: list[dict]) -> None:
    item = {"code": code, "pass": bool(condition), "message": message}
    checks.append(item)
    if not condition:
        failures.append(item)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()

    checks: list[dict] = []
    failures: list[dict] = []

    manifest_path = ROOT / "ggen.toml"
    check(manifest_path.is_file(), "GGEN-MANIFEST-001", "root ggen.toml exists", checks, failures)
    if manifest_path.is_file():
        try:
            manifest = tomllib.loads(manifest_path.read_text())
        except (OSError, tomllib.TOMLDecodeError) as exc:
            manifest = {}
            check(False, "GGEN-MANIFEST-002", f"root manifest parses: {exc}", checks, failures)
        else:
            check(True, "GGEN-MANIFEST-002", "root manifest parses", checks, failures)
        check("generation" not in manifest, "GGEN-SCHEMA-001", "legacy declarative generation table is absent", checks, failures)
        check(manifest.get("project", {}).get("name") == "wasm4pm-compat", "GGEN-SCHEMA-002", "consumer identity is exact", checks, failures)
        pack = manifest.get("packs", {}).get("wasm4pm-compat-pack", {})
        check(pack.get("path") == "packs/wasm4pm-compat-pack", "GGEN-PACK-001", "canonical local pack is selected", checks, failures)
        check(manifest.get("law", {}).get("reflexive") is True, "GGEN-RECEIPT-001", "reflexive receipt history is admitted", checks, failures)

    for path in SHADOW_MANIFESTS:
        check(not (ROOT / path).exists(), "GGEN-SHADOW-CONFIG-001", f"shadow manifest absent: {path}", checks, failures)
    check(not (ROOT / "ggen/.ggen/sync-state.json").exists(), "GGEN-STATE-001", "legacy mutable sync state is absent", checks, failures)

    pack_manifest = ROOT / "packs/wasm4pm-compat-pack/pack.toml"
    check(pack_manifest.is_file(), "GGEN-PACK-002", "pack.toml exists", checks, failures)
    if pack_manifest.is_file():
        pack_text = pack_manifest.read_text()
        check(f'version = "{GGEN_VERSION}"' in pack_text, "GGEN-PIN-001", f"pack pins ggen standard {GGEN_VERSION}", checks, failures)
    pack_ontology = ROOT / "packs/wasm4pm-compat-pack/ontology.ttl"
    check(pack_ontology.is_file(), "GGEN-PACK-003", "pack ontology exists", checks, failures)
    if pack_ontology.is_file():
        check(GGEN_COMMIT in pack_ontology.read_text(), "GGEN-PIN-002", "pack authority binds the exact ggen commit", checks, failures)

    templates = sorted((ROOT / "packs/wasm4pm-compat-pack/templates").glob("*.tmpl"))
    check(len(templates) >= 10, "GGEN-TEMPLATE-001", "active pack contains the full projection family", checks, failures)
    outputs: set[str] = set()
    for template in templates:
        text = template.read_text()
        relative = template.relative_to(ROOT).as_posix()
        check(text.startswith("---\n"), "GGEN-FRONTMATTER-001", f"frontmatter present: {relative}", checks, failures)
        match = re.search(r"(?m)^to:\s*(\S+)\s*$", text)
        check(match is not None, "GGEN-OUTPUT-001", f"output declared: {relative}", checks, failures)
        if match:
            output = match.group(1)
            safe = not output.startswith(("/", "~")) and ".." not in Path(output).parts
            check(safe, "GGEN-ACTUATION-001", f"repository-local output: {output}", checks, failures)
            check(output not in outputs, "GGEN-OUTPUT-002", f"single writer for output: {output}", checks, failures)
            outputs.add(output)
        check("freeze_policy: checksum" in text, "GGEN-FREEZE-001", f"checksum freeze enabled: {relative}", checks, failures)
        select_count = len(re.findall(r"(?im)^\s*SELECT\b", text))
        order_count = len(re.findall(r"(?im)\bORDER\s+BY\b", text))
        check(select_count == order_count and select_count >= 1, "GGEN-ORDER-001", f"every SELECT is ordered: {relative}", checks, failures)

    for path in ACTIVE_TEXT:
        file_path = ROOT / path
        if not file_path.exists():
            continue
        text = file_path.read_text(errors="replace")
        for code, pattern in FORBIDDEN_ACTIVE_PATTERNS.items():
            check(pattern.search(text) is None, code, f"active surface excludes deprecated form: {path}", checks, failures)

    report = {
        "schema": "https://chatmangpt.com/schemas/ggen-usage-audit/v1",
        "ggen_version": GGEN_VERSION,
        "ggen_commit": GGEN_COMMIT,
        "standing": "PARTIAL_ALIVE" if not failures else "BUILD_BROKEN",
        "check_count": len(checks),
        "failure_count": len(failures),
        "checks": checks,
        "failures": failures,
    }
    encoded = json.dumps(report, indent=2, sort_keys=True) + "\n"
    if args.output:
        output = args.output if args.output.is_absolute() else ROOT / args.output
        output.parent.mkdir(parents=True, exist_ok=True)
        output.write_text(encoded)
    print(encoded, end="")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
