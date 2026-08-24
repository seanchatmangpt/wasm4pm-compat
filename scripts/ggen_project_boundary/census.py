def census(observation: dict) -> dict:
    rules = observation["rules"]
    escaped = sum(1 for row in rules if row["target"]["failure_code"] == "FM-WRITE-002")
    missing_inputs = sum(1 for row in rules if "MISSING" in (row["query_digest"], row["template_digest"]))
    return {
        "active_rules": len(rules),
        "contained_rules": len(rules) - escaped,
        "escaped_rules": escaped,
        "missing_input_digests": missing_inputs,
        "disabled_cross_repo_targets": len(observation["disabled_cross_repo_targets"]),
    }
