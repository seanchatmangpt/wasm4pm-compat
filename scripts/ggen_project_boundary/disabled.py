import re


def disabled_cross_repo_targets(manifest_text: str) -> tuple[str, ...]:
    targets = []
    for line in manifest_text.splitlines():
        stripped = line.strip()
        if stripped.startswith("#") and "output_file" in stripped and ".." in stripped:
            match = re.search(r'output_file\s*=\s*"([^"]+)"', stripped.lstrip("# "))
            if match:
                targets.append(match.group(1))
    return tuple(sorted(set(targets)))
