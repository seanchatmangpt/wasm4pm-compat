use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct Group {
    source_module: String,
    elixir_namespace: String,
    rust_types: Vec<String>,
    subtype: String,
    projection_class: String,
    source_feature: String,
}

fn repo(path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
}

fn quoted(line: &str, predicate: &str) -> Option<String> {
    let marker = format!("ashbind:{predicate} \"");
    let rest = &line[line.find(&marker)? + marker.len()..];
    Some(rest[..rest.find('"')?].to_owned())
}

fn rust_types(line: &str) -> Option<Vec<String>> {
    let marker = "ashbind:rustTypes (";
    let rest = &line[line.find(marker)? + marker.len()..];
    let list = &rest[..rest.find(')')?];
    Some(
        list.split('"')
            .enumerate()
            .filter_map(|(i, part)| (i % 2 == 1).then_some(part.to_owned()))
            .collect(),
    )
}

fn parse_groups(ttl: &str) -> Vec<Group> {
    let mut groups = Vec::new();
    let mut current: Option<Group> = None;

    for line in ttl.lines() {
        if line.starts_with("ashbind:Group_") {
            if let Some(group) = current.take() {
                groups.push(group);
            }
            current = Some(Group {
                source_module: String::new(),
                elixir_namespace: String::new(),
                rust_types: Vec::new(),
                subtype: String::new(),
                projection_class: String::new(),
                source_feature: String::new(),
            });
            continue;
        }

        let Some(group) = current.as_mut() else {
            continue;
        };

        if let Some(v) = quoted(line, "sourceModule") {
            group.source_module = v;
        } else if let Some(v) = quoted(line, "elixirNamespace") {
            group.elixir_namespace = v;
        } else if let Some(v) = rust_types(line) {
            group.rust_types = v;
        } else if let Some(v) = quoted(line, "subtypeOf") {
            group.subtype = v;
        } else if let Some(v) = quoted(line, "projectionClass") {
            group.projection_class = v;
        } else if let Some(v) = quoted(line, "sourceFeature") {
            group.source_feature = v;
        }
    }
    if let Some(group) = current {
        groups.push(group);
    }
    groups
}

fn snake_case(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut out = String::new();

    for (i, &ch) in chars.iter().enumerate() {
        if ch.is_ascii_uppercase() {
            let prev_lower_or_digit =
                i > 0 && (chars[i - 1].is_ascii_lowercase() || chars[i - 1].is_ascii_digit());
            let acronym_boundary = i > 0
                && chars[i - 1].is_ascii_uppercase()
                && i + 1 < chars.len()
                && chars[i + 1].is_ascii_lowercase();
            if (prev_lower_or_digit || acronym_boundary) && !out.ends_with('_') {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

#[test]
fn ggen_ash_projection_is_closed_and_in_sync() {
    let manifest = fs::read_to_string(repo("ggen/ash-types.toml")).unwrap();
    let ttl = fs::read_to_string(repo("ggen/ontology/ash-types.ttl")).unwrap();
    let substrate = fs::read_to_string(repo("ggen/ontology/ash-substrate.ttl")).unwrap();
    let query = fs::read_to_string(repo("ggen/queries/extract-ash-types.rq")).unwrap();
    let template = fs::read_to_string(repo("ggen/templates/ash-types.ex.tera")).unwrap();
    let generated =
        fs::read_to_string(repo("bindings/elixir/lib/wasm4pm_compat/ash_types.ex")).unwrap();

    for needle in [
        "name = \"elixir-ash-types\"",
        "queries/extract-ash-types.rq",
        "templates/ash-types.ex.tera",
        "bindings/elixir/lib/wasm4pm_compat/ash_types.ex",
    ] {
        assert!(manifest.contains(needle), "manifest missing {needle}");
        if needle != "name = \"elixir-ash-types\"" {
            assert!(substrate.contains(needle), "substrate missing {needle}");
        }
    }

    for binding in [
        "sourceModule",
        "elixirNamespace",
        "rustType",
        "subtypeOf",
        "projectionClass",
        "sourceFeature",
    ] {
        assert!(
            query.contains(&format!("?{binding}")),
            "SPARQL missing {binding}"
        );
        assert!(
            template.contains(&format!("row.{binding}")),
            "Tera missing {binding}"
        );
    }
    assert!(query.contains("rdf:rest*/rdf:first"));

    let groups = parse_groups(&ttl);
    assert_eq!(groups.len(), 31);

    let admitted_subtypes = ["map", "term", "atom", "string", "integer", "binary"];
    let mut short_names = HashSet::new();
    let mut modules = HashSet::new();
    let mut members = 0usize;

    for group in &groups {
        assert!(!group.source_module.is_empty(), "{group:?}");
        assert!(!group.elixir_namespace.is_empty(), "{group:?}");
        assert!(!group.rust_types.is_empty(), "{group:?}");
        assert!(admitted_subtypes.contains(&group.subtype.as_str()));

        for rust_type in &group.rust_types {
            members += 1;
            let short_name = format!("wasm4pm_{}", snake_case(rust_type));
            let module = format!("{}.{}", group.elixir_namespace, rust_type);
            assert!(
                short_names.insert(short_name.clone()),
                "duplicate {short_name}"
            );
            assert!(modules.insert(module.clone()), "duplicate {module}");

            let row = format!(
                "{{:{short_name}, {module}, \"{}\", \"{rust_type}\", :{}, :{}, :{}",
                group.source_module, group.subtype, group.projection_class, group.source_feature
            );
            assert!(generated.contains(&row), "generated row drift: {row}");
        }
    }

    assert_eq!(members, 71);
    assert_eq!(generated.matches("    {:wasm4pm_").count(), 71);
    assert_eq!(
        generated
            .matches("use Ash.Type.NewType, subtype_of:")
            .count(),
        1
    );
    assert!(generated.contains("Module.create(module, body, Macro.Env.location(__ENV__))"));
    assert!(ttl.contains("does not recreate Rust typestate"));
    assert!(generated.contains("they do not\n  manufacture Rust typestate"));
}
