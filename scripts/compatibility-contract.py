#!/usr/bin/env python3
"""Verify and compare wasm4pm-compat's public compatibility contract.

This verifier intentionally uses only the Python standard library so it can run
before Rust toolchain installation. It issues PARTIAL_ALIVE at most; exact-tree
ALIVE remains external.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
import tomllib
from pathlib import Path
from typing import Any

EXIT_OK = 0
EXIT_DRIFT = 2
EXIT_USAGE = 64


def canonical_json(value: Any) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def sha256_json(value: Any) -> str:
    return hashlib.sha256(canonical_json(value).encode("utf-8")).hexdigest()


def load_json(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(value, dict):
        raise ValueError(f"{path} must contain a JSON object")
    return value


def read_toml(path: Path) -> dict[str, Any]:
    return tomllib.loads(path.read_text(encoding="utf-8"))


def public_modules(source: str) -> set[str]:
    return set(re.findall(r"(?m)^\s*pub\s+mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*;", source))


def prelude_exports(source: str) -> set[str]:
    exports: set[str] = set()
    for match in re.finditer(r"pub\s+use\s+.*?;", source, flags=re.DOTALL):
        statement = match.group(0)
        exports.update(re.findall(r"\bas\s+([A-Za-z_][A-Za-z0-9_]*)", statement))
        exports.update(re.findall(r"\b([A-Z][A-Za-z0-9_]*)\b", statement))
    return exports


def scan_forbidden(root: Path, tokens: list[str]) -> list[dict[str, Any]]:
    matches: list[dict[str, Any]] = []
    for path in sorted((root / "src").rglob("*.rs")):
        text = path.read_text(encoding="utf-8")
        for line_number, line in enumerate(text.splitlines(), start=1):
            for token in tokens:
                if token in line:
                    matches.append(
                        {
                            "path": path.relative_to(root).as_posix(),
                            "line": line_number,
                            "token": token,
                        }
                    )
    return matches


def observe(root: Path, contract: dict[str, Any]) -> dict[str, Any]:
    cargo = read_toml(root / "Cargo.toml")
    toolchain = read_toml(root / "rust-toolchain.toml")
    lib_source = (root / "src/lib.rs").read_text(encoding="utf-8")
    prelude_source = (root / "src/prelude.rs").read_text(encoding="utf-8")
    doctor_source = (root / "src/diagnostic/doctor.rs").read_text(encoding="utf-8")

    features = cargo.get("features", {})
    public_feature_names = sorted(name for name in features if name != "default")
    default_features = list(features.get("default", []))

    capability_codes = [item["code"] for item in contract["capabilities"]]
    observed_capabilities = sorted(
        code for code in capability_codes if f'"{code}"' in doctor_source
    )

    authority = {
        "execution_owner": "wasm4pm"
        if "RouteTarget::Wasm4pm" in doctor_source
        else None,
        "standing_owner": "external-verifier"
        if "RouteTarget::ExternalVerifier" in doctor_source
        else None,
        "max_self_issued_standing": "PARTIAL_ALIVE"
        if "DoctorStanding::PartialAlive" in doctor_source
        and (
            standing_match := re.search(
                r"pub\s+enum\s+DoctorStanding\s*\{(?P<body>.*?)\}",
                doctor_source,
                flags=re.DOTALL,
            )
        )
        and not re.search(r"(?m)^\s*Alive\s*(?:,|$)", standing_match.group("body"))
        else None,
    }

    return {
        "package": {
            "name": cargo["package"]["name"],
            "version": cargo["package"]["version"],
            "edition": cargo["package"]["edition"],
            "toolchain": toolchain["toolchain"]["channel"],
            "scope": "structure-only" if "structure-only" in lib_source.lower() else None,
        },
        "features": {
            "public": public_feature_names,
            "default": default_features,
        },
        "authority": authority,
        "modules": sorted(public_modules(lib_source)),
        "prelude_exports": sorted(prelude_exports(prelude_source)),
        "capabilities": observed_capabilities,
        "forbidden_matches": scan_forbidden(root, contract["forbidden_source_tokens"]),
    }


def compare_observation(
    contract: dict[str, Any], observation: dict[str, Any]
) -> list[dict[str, Any]]:
    failures: list[dict[str, Any]] = []

    for field in ("name", "version", "edition", "toolchain", "scope"):
        expected = contract["package"][field]
        actual = observation["package"][field]
        if actual != expected:
            failures.append(
                {
                    "code": f"PACKAGE_{field.upper()}_DRIFT",
                    "expected": expected,
                    "actual": actual,
                }
            )

    for field in ("public", "default"):
        expected = sorted(contract["features"][field])
        actual = sorted(observation["features"][field])
        if actual != expected:
            failures.append(
                {
                    "code": f"FEATURE_{field.upper()}_DRIFT",
                    "expected": expected,
                    "actual": actual,
                }
            )

    for field, expected in contract["authority"].items():
        actual = observation["authority"].get(field)
        if actual != expected:
            failures.append(
                {
                    "code": f"AUTHORITY_{field.upper()}_DRIFT",
                    "expected": expected,
                    "actual": actual,
                }
            )

    missing_modules = sorted(set(contract["required_modules"]) - set(observation["modules"]))
    if missing_modules:
        failures.append({"code": "REQUIRED_MODULES_MISSING", "missing": missing_modules})

    missing_exports = sorted(
        set(contract["required_prelude_exports"]) - set(observation["prelude_exports"])
    )
    if missing_exports:
        failures.append({"code": "PRELUDE_EXPORTS_MISSING", "missing": missing_exports})

    expected_capabilities = sorted(item["code"] for item in contract["capabilities"])
    if observation["capabilities"] != expected_capabilities:
        failures.append(
            {
                "code": "CAPABILITY_INVENTORY_DRIFT",
                "expected": expected_capabilities,
                "actual": observation["capabilities"],
            }
        )

    if observation["forbidden_matches"]:
        failures.append(
            {
                "code": "FORBIDDEN_SOURCE_TOKEN",
                "matches": observation["forbidden_matches"],
            }
        )

    return failures


def check_contract(root: Path, contract_path: Path) -> dict[str, Any]:
    contract = load_json(contract_path)
    observation = observe(root, contract)
    failures = compare_observation(contract, observation)
    return {
        "schema": "wasm4pm-compat/compatibility-check/v1",
        "standing": "PARTIAL_ALIVE" if not failures else "BLOCKED",
        "contract_sha256": sha256_json(contract),
        "observation_sha256": sha256_json(observation),
        "failure_count": len(failures),
        "failures": failures,
        "observation": observation,
    }


def capability_map(contract: dict[str, Any]) -> dict[str, str]:
    return {item["code"]: item["owner"] for item in contract["capabilities"]}


def classify_contract_change(
    old: dict[str, Any], new: dict[str, Any]
) -> dict[str, Any]:
    breaking: list[dict[str, Any]] = []
    compatible: list[dict[str, Any]] = []
    informational: list[dict[str, Any]] = []

    def breaking_change(code: str, before: Any, after: Any) -> None:
        breaking.append({"code": code, "before": before, "after": after})

    if old["schema_version"] != new["schema_version"]:
        breaking_change("SCHEMA_VERSION_CHANGED", old["schema_version"], new["schema_version"])
    if old["package"]["name"] != new["package"]["name"]:
        breaking_change("PACKAGE_NAME_CHANGED", old["package"]["name"], new["package"]["name"])
    if old["package"]["version"] != new["package"]["version"]:
        informational.append(
            {
                "code": "PACKAGE_VERSION_CHANGED",
                "before": old["package"]["version"],
                "after": new["package"]["version"],
            }
        )

    for field, code in (
        ("public", "PUBLIC_FEATURE_SET_CHANGED"),
        ("default", "DEFAULT_FEATURE_SET_CHANGED"),
    ):
        before = sorted(old["features"][field])
        after = sorted(new["features"][field])
        if before != after:
            breaking_change(code, before, after)

    for field, removal_code, addition_code in (
        ("required_modules", "REQUIRED_MODULE_REMOVED", "REQUIRED_MODULE_ADDED"),
        ("required_prelude_exports", "PRELUDE_EXPORT_REMOVED", "PRELUDE_EXPORT_ADDED"),
    ):
        before = set(old[field])
        after = set(new[field])
        for item in sorted(before - after):
            breaking_change(removal_code, item, None)
        for item in sorted(after - before):
            compatible.append({"code": addition_code, "before": None, "after": item})

    old_capabilities = capability_map(old)
    new_capabilities = capability_map(new)
    if set(old_capabilities) != set(new_capabilities):
        breaking_change(
            "CAPABILITY_SET_CHANGED",
            sorted(old_capabilities),
            sorted(new_capabilities),
        )
    for code in sorted(set(old_capabilities) & set(new_capabilities)):
        if old_capabilities[code] != new_capabilities[code]:
            breaking_change(
                "CAPABILITY_OWNER_CHANGED",
                {code: old_capabilities[code]},
                {code: new_capabilities[code]},
            )

    if old["authority"] != new["authority"]:
        breaking_change("AUTHORITY_CHANGED", old["authority"], new["authority"])

    old_forbidden = set(old["forbidden_source_tokens"])
    new_forbidden = set(new["forbidden_source_tokens"])
    for token in sorted(old_forbidden - new_forbidden):
        breaking_change("FORBIDDEN_TOKEN_RELAXED", token, None)
    for token in sorted(new_forbidden - old_forbidden):
        compatible.append({"code": "FORBIDDEN_TOKEN_ADDED", "before": None, "after": token})

    return {
        "schema": "wasm4pm-compat/compatibility-diff/v1",
        "classification": "breaking" if breaking else "compatible",
        "breaking": breaking,
        "compatible": compatible,
        "informational": informational,
        "old_sha256": sha256_json(old),
        "new_sha256": sha256_json(new),
    }


def write_result(result: dict[str, Any], output: Path | None) -> None:
    rendered = json.dumps(result, sort_keys=True, indent=2, ensure_ascii=False) + "\n"
    if output is not None:
        output.parent.mkdir(parents=True, exist_ok=True)
        output.write_text(rendered, encoding="utf-8")
    print(rendered, end="")


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    subparsers = parser.add_subparsers(dest="command", required=True)

    check = subparsers.add_parser("check", help="verify the repository against a contract")
    check.add_argument("--root", type=Path, default=Path("."))
    check.add_argument(
        "--contract", type=Path, default=Path("compatibility/contract-v1.json")
    )
    check.add_argument("--output", type=Path)

    diff = subparsers.add_parser("diff", help="classify two contract revisions")
    diff.add_argument("old", type=Path)
    diff.add_argument("new", type=Path)
    diff.add_argument("--output", type=Path)

    return parser


def main(argv: list[str] | None = None) -> int:
    args = build_parser().parse_args(argv)
    try:
        if args.command == "check":
            result = check_contract(args.root.resolve(), args.contract.resolve())
            write_result(result, args.output)
            return EXIT_OK if result["failure_count"] == 0 else EXIT_DRIFT
        if args.command == "diff":
            result = classify_contract_change(load_json(args.old), load_json(args.new))
            write_result(result, args.output)
            return EXIT_OK if result["classification"] == "compatible" else EXIT_DRIFT
    except (OSError, ValueError, KeyError, tomllib.TOMLDecodeError) as error:
        print(f"compatibility-contract: {error}", file=sys.stderr)
        return EXIT_USAGE
    return EXIT_USAGE


if __name__ == "__main__":
    raise SystemExit(main())
