#!/usr/bin/env python3
"""Audit every frozen branch/file object for stub or vacuous implementation signals.

The audit subject is the exact branch->commit map in
.github/vacuity-branch-snapshot.json.  Identical Git blobs are inspected once while
all branch/path occurrences remain in the report.  Findings are candidates for
semantic review; this tool never rewrites source and never promotes standing.
"""
from __future__ import annotations

import argparse
import json
import re
import subprocess
from collections import defaultdict
from pathlib import Path
from typing import Any

IMPLEMENTATION_SUFFIXES = {
    ".rs", ".py", ".pyi", ".js", ".jsx", ".ts", ".tsx", ".sh", ".bash",
    ".zsh", ".yml", ".yaml", ".toml", ".json", ".tera", ".rq", ".sparql",
}
GENERATED_PATHS = {
    "tests/fixtures/ggen_standing_projection.rs",
    "tests/fixtures/ggen_gall_checkpoints.rs",
}
GENERATED_PREFIXES = (".ggen/cache/", ".ggen/receipts/")

RULES: list[tuple[str, str, str, re.Pattern[str]]] = [
    ("rust_todo", "high", "Rust todo! macro", re.compile(r"\btodo!\s*\(")),
    ("rust_unimplemented", "high", "Rust unimplemented! macro", re.compile(r"\bunimplemented!\s*\(")),
    ("placeholder_panic", "high", "placeholder panic", re.compile(r"panic!\s*\([^\n]*(?:not implemented|todo|stub|placeholder)", re.I)),
    ("python_not_implemented", "high", "Python NotImplementedError", re.compile(r"\braise\s+NotImplementedError\b")),
    ("python_pass", "medium", "standalone Python pass", re.compile(r"^\s*pass\s*(?:#.*)?$", re.M)),
    ("python_ellipsis", "medium", "standalone Python ellipsis", re.compile(r"^\s*\.\.\.\s*(?:#.*)?$", re.M)),
    ("vacuous_assert", "high", "vacuous always-true assertion", re.compile(r"(?:assert!\s*\(\s*true\s*\)|assert_eq!\s*\(\s*(true|false)\s*,\s*\1\s*\)|assert\s+True\b|expect\s*\(\s*true\s*\))")),
    ("failure_mask", "high", "failure masking with || true", re.compile(r"\|\|\s*true\b")),
    ("continue_on_error", "high", "GitHub Actions continue-on-error", re.compile(r"^\s*continue-on-error\s*:\s*true\s*$", re.I | re.M)),
    ("placeholder_marker", "medium", "explicit placeholder/stub marker", re.compile(r"\b(?:TODO|FIXME|XXX|STUB|PLACEHOLDER)\b", re.I)),
    ("not_implemented_text", "medium", "explicit not-implemented marker", re.compile(r"\bnot[- ]implemented\b", re.I)),
    ("suspicious_unreachable", "review", "unreachable macro requiring semantic review", re.compile(r"\bunreachable!\s*\(")),
    ("ignored_rust_test", "review", "ignored Rust test requiring justification", re.compile(r"#\s*\[\s*ignore(?:\s*=|\s*\])")),
]

SUSPICIOUS_RETURN = re.compile(
    r"\breturn\s+(?:Ok\s*\(\s*\(\s*\)\s*\)|Ok\s*\(\s*Default::default\s*\(\s*\)\s*\)|"
    r"Default::default\s*\(\s*\)|None|true|false|Vec::new\s*\(\s*\)|String::new\s*\(\s*\))\s*;"
)


def git(*args: str, text: bool = False) -> bytes | str:
    proc = subprocess.run(["git", *args], check=False, capture_output=True)
    if proc.returncode != 0:
        raise RuntimeError(
            f"git {' '.join(args)} failed ({proc.returncode}): "
            f"{proc.stderr.decode('utf-8', 'replace').strip()}"
        )
    return proc.stdout.decode("utf-8", "replace") if text else proc.stdout


def line_number(text: str, offset: int) -> int:
    return text.count("\n", 0, offset) + 1


def line_excerpt(text: str, offset: int) -> str:
    start = text.rfind("\n", 0, offset) + 1
    end = text.find("\n", offset)
    if end < 0:
        end = len(text)
    return text[start:end].strip()[:240]


def path_class(path: str) -> str:
    if path in GENERATED_PATHS or path.startswith(GENERATED_PREFIXES):
        return "generated_projection"
    suffix = Path(path).suffix.lower()
    if suffix in IMPLEMENTATION_SUFFIXES or path.startswith(".github/workflows/"):
        return "implementation"
    return "text_support"


def scan_text(path: str, text: str) -> list[dict[str, Any]]:
    if path_class(path) == "text_support":
        return []
    findings: list[dict[str, Any]] = []
    for rule, severity, description, pattern in RULES:
        for match in pattern.finditer(text):
            findings.append({
                "rule": rule,
                "severity": severity,
                "description": description,
                "line": line_number(text, match.start()),
                "excerpt": line_excerpt(text, match.start()),
            })
    # Constant/default returns are intentionally review-only: they are sometimes lawful
    # constructors/refusals, but any implementation body using one deserves inspection.
    for match in SUSPICIOUS_RETURN.finditer(text):
        findings.append({
            "rule": "constant_default_return",
            "severity": "review",
            "description": "constant/default return requiring contract review",
            "line": line_number(text, match.start()),
            "excerpt": line_excerpt(text, match.start()),
        })
    return findings


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--snapshot", default=".github/vacuity-branch-snapshot.json")
    parser.add_argument("--json", default="target/vacuity-audit.json")
    parser.add_argument("--markdown", default="target/vacuity-audit.md")
    args = parser.parse_args()

    snapshot = json.loads(Path(args.snapshot).read_text(encoding="utf-8"))
    branches: dict[str, str] = snapshot["branches"]
    unique_tips = sorted(set(branches.values()))

    for sha in unique_tips:
        git("cat-file", "-e", f"{sha}^{{commit}}")

    occurrences: dict[str, list[dict[str, str]]] = defaultdict(list)
    branch_counts: dict[str, int] = {}
    for branch, tip in branches.items():
        raw = git("ls-tree", "-r", "-z", tip)
        assert isinstance(raw, bytes)
        entries = [entry for entry in raw.split(b"\0") if entry]
        branch_counts[branch] = len(entries)
        for entry in entries:
            meta, path_b = entry.split(b"\t", 1)
            mode, obj_type, blob = meta.decode("ascii").split()
            if obj_type != "blob":
                continue
            path = path_b.decode("utf-8", "surrogateescape")
            occurrences[blob].append({"branch": branch, "tip": tip, "path": path, "mode": mode})

    findings: list[dict[str, Any]] = []
    blob_inventory: list[dict[str, Any]] = []
    text_blobs = 0
    binary_blobs = 0
    total_bytes = 0

    for blob in sorted(occurrences):
        content = git("cat-file", "blob", blob)
        assert isinstance(content, bytes)
        total_bytes += len(content)
        try:
            text = content.decode("utf-8")
            is_binary = "\x00" in text
        except UnicodeDecodeError:
            text = ""
            is_binary = True

        paths = sorted({item["path"] for item in occurrences[blob]})
        classes = sorted({path_class(path) for path in paths})
        blob_inventory.append({
            "blob": blob,
            "size": len(content),
            "binary": is_binary,
            "classes": classes,
            "occurrence_count": len(occurrences[blob]),
            "paths": paths,
        })
        if is_binary:
            binary_blobs += 1
            continue
        text_blobs += 1
        # A blob can occur under multiple paths. Scan once per distinct path because path
        # class matters, then attach every exact branch/path occurrence for that path.
        for path in paths:
            for candidate in scan_text(path, text):
                candidate["blob"] = blob
                candidate["path"] = path
                candidate["path_class"] = path_class(path)
                candidate["occurrences"] = [
                    item for item in occurrences[blob] if item["path"] == path
                ]
                findings.append(candidate)

    severity_rank = {"high": 0, "medium": 1, "review": 2}
    findings.sort(key=lambda item: (severity_rank[item["severity"]], item["path"], item["line"], item["rule"]))

    report = {
        "schema": "wasm4pm-compat.vacuity-audit.v1",
        "repository": snapshot["repository"],
        "canonical_base": snapshot["canonical_base"],
        "captured_at": snapshot["captured_at"],
        "branch_refs": len(branches),
        "unique_tip_commits": len(unique_tips),
        "branch_file_occurrences": sum(branch_counts.values()),
        "unique_blobs": len(occurrences),
        "text_blobs_inspected": text_blobs,
        "binary_blobs_classified": binary_blobs,
        "bytes_inspected": total_bytes,
        "branch_file_counts": branch_counts,
        "finding_count": len(findings),
        "findings": findings,
        "blob_inventory": blob_inventory,
    }

    json_path = Path(args.json)
    json_path.parent.mkdir(parents=True, exist_ok=True)
    json_path.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    by_severity = defaultdict(int)
    for finding in findings:
        by_severity[finding["severity"]] += 1
    md = [
        "# Exhaustive vacuity audit",
        "",
        f"- Branch refs: **{len(branches)}**",
        f"- Unique frozen tips: **{len(unique_tips)}**",
        f"- Branch/file occurrences: **{sum(branch_counts.values())}**",
        f"- Unique blobs: **{len(occurrences)}**",
        f"- UTF-8 text blobs inspected: **{text_blobs}**",
        f"- Binary blobs classified: **{binary_blobs}**",
        f"- Bytes inspected: **{total_bytes}**",
        f"- Findings: **{len(findings)}** (high={by_severity['high']}, medium={by_severity['medium']}, review={by_severity['review']})",
        "",
        "## Candidates",
        "",
    ]
    for finding in findings:
        occ = finding["occurrences"]
        tips = ", ".join(sorted({f"{x['branch']}@{x['tip'][:12]}" for x in occ}))
        md.append(
            f"- **{finding['severity'].upper()} {finding['rule']}** "
            f"`{finding['path']}:{finding['line']}` `{finding['blob'][:12]}` — "
            f"{finding['excerpt']} — {tips}"
        )
    if not findings:
        md.append("No static candidates. This is not by itself an ALIVE claim.")
    markdown_path = Path(args.markdown)
    markdown_path.parent.mkdir(parents=True, exist_ok=True)
    markdown_path.write_text("\n".join(md) + "\n", encoding="utf-8")

    print("\n".join(md))
    # Candidates require semantic classification, so their presence is not an execution
    # failure. Object/read failures above do fail the audit.
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
