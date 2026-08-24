from .paths import classify_path
from .types import TargetObservation


def observe_target(rule: str, output_file: str, source_repo: str, target_repo: str, isolated_stage: bool = False) -> TargetObservation:
    absolute, traversal, contained = classify_path(output_file)
    cross_repo = source_repo != target_repo
    failure = None
    if not contained or (cross_repo and not isolated_stage):
        failure = "FM-WRITE-002"
    return TargetObservation(rule, output_file, absolute, traversal, contained, source_repo, target_repo, isolated_stage, failure)
