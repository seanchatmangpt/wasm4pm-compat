from pathlib import Path
import tomllib

from .digests import file_digest
from .disabled import disabled_cross_repo_targets
from .target import observe_target
from .types import Refused, Subject


def inspect_manifest(root: Path, subject: Subject, manifest_rel: str = "ggen/ggen.toml") -> dict:
    manifest_path = root / manifest_rel
    data = tomllib.loads(manifest_path.read_text())
    generation = data.get("generation", {})
    output_dir = str(generation.get("output_dir", "."))
    if output_dir.startswith("/") or ".." in Path(output_dir).parts:
        raise Refused("REFUSED[INVALID_OUTPUT_DIR]")
    rows = []
    for rule in generation.get("rules", []):
        name = str(rule["name"])
        output_file = str(rule["output_file"])
        observation = observe_target(name, output_file, subject.repo, subject.repo)
        query = rule.get("query", {})
        template = rule.get("template", {})
        query_file = query.get("file") if isinstance(query, dict) else query
        template_file = template.get("file") if isinstance(template, dict) else template
        rows.append({
            "rule": name,
            "target": observation.__dict__,
            "query_digest": file_digest(root, str(query_file)) if query_file else "MISSING",
            "template_digest": file_digest(root, str(template_file)) if template_file else "MISSING",
        })
    return {
        "subject": subject.__dict__,
        "manifest": manifest_rel,
        "output_dir": output_dir,
        "rules": rows,
        "disabled_cross_repo_targets": disabled_cross_repo_targets(manifest_path.read_text()),
        "authority": "OBSERVE|VERIFY",
        "actuation_performed": False,
    }
