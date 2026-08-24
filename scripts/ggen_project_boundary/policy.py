from .types import Refused


def admission_for_target(target: dict) -> str:
    if target["failure_code"] == "FM-WRITE-002":
        return "REFUSED[PROJECT_ROOT_ESCAPE]"
    if target["source_repo"] != target["target_repo"] and not target["isolated_stage"]:
        raise Refused("REFUSED[CROSS_REPO_WITHOUT_ISOLATION]")
    return "ADMITTED"
