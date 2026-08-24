def to_turtle(observation: dict) -> str:
    lines = [
        "@prefix gpb: <https://ggen.dev/ontology/project-boundary#> .",
        "@prefix ex: <https://wasm4pm.dev/evidence/> .",
    ]
    subject = observation["subject"]
    lines.append(f'ex:boundary a gpb:ProjectBoundary ; gpb:exactSubject "{subject["sha"]}" ; gpb:ggenSubject "{subject["ggen_sha"]}" ; gpb:projectRoot "." ; gpb:manifestPath "{observation["manifest"]}" ; gpb:outputDir "{observation["output_dir"]}" ; gpb:authority "OBSERVE|VERIFY" ; gpb:actuationPerformed false .')
    for idx, row in enumerate(observation["rules"]):
        t = row["target"]
        lines.append(f'ex:target{idx} a gpb:GenerationTarget ; gpb:sourceRepository "{t["source_repo"]}" ; gpb:targetRepository "{t["target_repo"]}" ; gpb:outputFile "{t["output_file"]}" ; gpb:contained {str(t["contained"]).lower()} ; gpb:pathTraversal {str(t["traversal"]).lower()} ; gpb:absolutePath {str(t["absolute"]).lower()} ; gpb:isolatedStage {str(t["isolated_stage"]).lower()} ; gpb:queryDigest "{row["query_digest"]}" ; gpb:templateDigest "{row["template_digest"]}" .')
    return "\n".join(lines) + "\n"
