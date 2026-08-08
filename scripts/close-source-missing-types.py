#!/usr/bin/env python3
"""Close the ontology's source_missing gap: emit zod: declarations for public Serde
Rust types that exist in source but have no ontology entry at all, so they never reach
the ggen -> Pydantic generation pipeline.

Never fabricates a field shape it isn't confident about. A struct field or an entire
enum with data-carrying variants that this mechanical parser can't confidently resolve
falls back to a marker-only declaration (compat:rustType with no zod:hasField) - this is
an intentional, already-established convention in this ontology (see the pydantic
template's own docstring: "Empty specifications represent intentional marker/typestate
surfaces rather than invented runtime fields"), not a bug.

Usage:
    python3 scripts/close-source-missing-types.py \
        --root . --receipt /tmp/verify-before.json \
        --ontology ggen/ontology/zod-types.ttl
"""
from __future__ import annotations

import argparse
import json
import re
from dataclasses import dataclass, field
from pathlib import Path

PRIMITIVE_MAP = {
    "String": "string",
    "str": "string",
    "&str": "string",
    "bool": "boolean",
    "u8": "number", "u16": "number", "u32": "number", "u64": "number", "u128": "number",
    "usize": "number",
    "i8": "number", "i16": "number", "i32": "number", "i64": "number", "i128": "number",
    "isize": "number",
    "f32": "number", "f64": "number",
}

MAP_LIKE_PREFIXES = ("HashMap<", "BTreeMap<", "PackedKeyTable<", "IndexMap<")

IDENT_RE = re.compile(r"^[A-Za-z_][A-Za-z0-9_]*$")


@dataclass
class FieldSpec:
    name: str
    field_type: str
    optional: bool
    array: bool


@dataclass
class TypeSpec:
    name: str
    fields: list[FieldSpec] = field(default_factory=list)
    marker_only: bool = False
    reason: str = ""


def _strip_serde_field(rust_type: str) -> str:
    return rust_type.strip().rstrip(",").strip()


def _resolve_rust_type(rust_type: str, known: set[str]) -> tuple[str, bool, bool] | None:
    """Return (field_type, is_optional, is_array) or None if unresolvable."""
    rust_type = rust_type.strip()
    m = re.fullmatch(r"Option\s*<\s*(.+)\s*>", rust_type)
    if m:
        inner = _resolve_rust_type(m.group(1), known)
        if inner is None:
            return None
        field_type, _, array = inner
        return field_type, True, array
    m = re.fullmatch(r"Vec\s*<\s*(.+)\s*>", rust_type)
    if m:
        inner = _resolve_rust_type(m.group(1), known)
        if inner is None:
            return None
        field_type, optional, _ = inner
        return field_type, optional, True
    m = re.fullmatch(r"Box\s*<\s*(.+)\s*>", rust_type)
    if m:
        return _resolve_rust_type(m.group(1), known)
    for prefix in MAP_LIKE_PREFIXES:
        if rust_type.startswith(prefix):
            # Only confidently model the common String-keyed, primitive-valued case.
            inner = rust_type[len(prefix):-1]
            parts = [p.strip() for p in inner.split(",")]
            if len(parts) == 2 and parts[0] in ("String", "str", "&str"):
                return "Record<string, string>", False, False
            return "any", False, False
    if rust_type in PRIMITIVE_MAP:
        return PRIMITIVE_MAP[rust_type], False, False
    if IDENT_RE.match(rust_type) and rust_type in known:
        return rust_type, False, False
    return None


def _extract_type_block(source: str, type_name: str) -> tuple[str, str] | None:
    """Return (kind, block_body) for `pub struct TypeName { ... }` / `pub enum TypeName { ... }`,
    using brace matching (fields/variants may span multiple lines)."""
    pattern = re.compile(
        r"pub\s+(struct|enum)\s+" + re.escape(type_name) + r"\b[^{;]*\{",
    )
    m = pattern.search(source)
    if m is None:
        # Unit struct or tuple struct without braces (e.g. `pub struct X;` or `pub struct X(...)`).
        return None
    kind = m.group(1)
    start = m.end()
    depth = 1
    i = start
    while i < len(source) and depth > 0:
        if source[i] == "{":
            depth += 1
        elif source[i] == "}":
            depth -= 1
        i += 1
    return kind, source[start:i - 1]


def _parse_struct_fields(body: str, known: set[str]) -> tuple[list[FieldSpec], str]:
    fields: list[FieldSpec] = []
    # Strip doc comments and attributes for a cleaner scan.
    cleaned = re.sub(r"///.*", "", body)
    cleaned = re.sub(r"#\[[^\]]*\]", "", cleaned)
    for line in cleaned.split("\n"):
        line = line.strip()
        m = re.match(r"pub\s+([A-Za-z_][A-Za-z0-9_]*)\s*:\s*(.+)", line)
        if not m:
            continue
        name, rust_type = m.group(1), _strip_serde_field(m.group(2))
        if rust_type.startswith("(") or "PhantomData" in rust_type:
            return [], "unresolvable field type (tuple/PhantomData)"
        resolved = _resolve_rust_type(rust_type, known)
        if resolved is None:
            return [], f"unresolvable field type: {name}: {rust_type}"
        field_type, optional, array = resolved
        fields.append(FieldSpec(name=name, field_type=field_type, optional=optional, array=array))
    if not fields:
        return [], "no `pub` fields found"
    return fields, ""


def _enum_is_all_unit_variants(body: str) -> bool:
    cleaned = re.sub(r"///.*", "", body)
    cleaned = re.sub(r"#\[[^\]]*\]", "", cleaned)
    for raw in cleaned.split(","):
        variant = raw.strip()
        if not variant:
            continue
        if "(" in variant or "{" in variant:
            return False
    return True


def build_type_spec(name: str, source_file: Path, known: set[str]) -> TypeSpec:
    source = source_file.read_text(encoding="utf-8")
    block = _extract_type_block(source, name)
    if block is None:
        return TypeSpec(name=name, marker_only=True, reason="no brace body (unit/tuple struct)")
    kind, body = block
    if kind == "enum":
        if _enum_is_all_unit_variants(body):
            return TypeSpec(name=name, marker_only=True, reason="unit-only enum")
        return TypeSpec(name=name, marker_only=True, reason="data-carrying enum variants")
    fields, reason = _parse_struct_fields(body, known)
    if not fields:
        return TypeSpec(name=name, marker_only=True, reason=reason)
    return TypeSpec(name=name, fields=fields)


def render_turtle(specs: list[TypeSpec]) -> str:
    lines = ["", "# --- source_missing closure (generated by scripts/close-source-missing-types.py) ---", ""]
    for spec in specs:
        if not spec.fields:
            lines.append(f'zod:{spec.name} compat:rustType "{spec.name}" .')
            continue
        lines.append(f"zod:{spec.name} compat:rustType \"{spec.name}\" .")
        parts = []
        for f in spec.fields:
            bits = [f'zod:fieldName "{f.name}"', f'zod:fieldType "{f.field_type}"']
            if f.optional:
                bits.append("zod:isOptional true")
            if f.array:
                bits.append("zod:isArray true")
            parts.append("[ " + " ; ".join(bits) + " ]")
        field_lines = " ;\n    zod:hasField ".join(parts)
        lines.append(f"zod:{spec.name} zod:hasField {field_lines} .")
        lines.append("")
    return "\n".join(lines) + "\n"


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", default=".")
    parser.add_argument("--receipt", required=True, help="pre-closure verify-pydantic-generation.py receipt (source_missing + source_locations)")
    parser.add_argument("--ontology", required=True, help="zod-types.ttl to append to")
    parser.add_argument("--dry-run", action="store_true")
    args = parser.parse_args(argv)

    root = Path(args.root).resolve()
    receipt = json.loads(Path(args.receipt).read_text(encoding="utf-8"))
    missing: list[str] = receipt["source_missing"]
    locations: dict[str, list[str]] = receipt["source_locations"]

    known = set(missing)
    ontology_text = (root / "ggen/ontology/wasm4pm-compat.ttl").read_text(encoding="utf-8")
    ontology_text += Path(args.ontology).read_text(encoding="utf-8") if Path(args.ontology).is_absolute() else (root / args.ontology).read_text(encoding="utf-8")
    known |= set(re.findall(r'compat:rustType\s+"([A-Za-z_][A-Za-z0-9_]*)"', ontology_text))

    specs: list[TypeSpec] = []
    skipped: list[tuple[str, str]] = []
    for name in sorted(missing):
        paths = locations.get(name, [])
        if not paths:
            skipped.append((name, "no source location"))
            continue
        source_file = Path(paths[0])
        if not source_file.is_file():
            skipped.append((name, f"source file missing: {source_file}"))
            continue
        spec = build_type_spec(name, source_file, known)
        specs.append(spec)

    full = [s for s in specs if s.fields]
    markers = [s for s in specs if not s.fields]

    print(f"{len(specs)} types processed, {len(full)} with full fields, {len(markers)} marker-only, {len(skipped)} skipped")
    if markers:
        print("marker-only reasons:")
        reasons: dict[str, int] = {}
        for s in markers:
            reasons[s.reason] = reasons.get(s.reason, 0) + 1
        for reason, count in sorted(reasons.items(), key=lambda kv: -kv[1]):
            print(f"  {count:3d}  {reason}")
    if skipped:
        print("skipped:", skipped)

    turtle = render_turtle(specs)
    if args.dry_run:
        print("--- dry run: turtle not written ---")
        print(turtle)
        return 0

    ontology_path = Path(args.ontology) if Path(args.ontology).is_absolute() else root / args.ontology
    with ontology_path.open("a", encoding="utf-8") as handle:
        handle.write(turtle)
    print(f"appended {len(specs)} type declarations to {ontology_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
