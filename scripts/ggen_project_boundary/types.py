from dataclasses import dataclass
from pathlib import Path
import re


class Refused(ValueError):
    pass


@dataclass(frozen=True)
class Subject:
    repo: str
    sha: str
    ggen_sha: str

    def __post_init__(self) -> None:
        if self.repo != "seanchatmangpt/wasm4pm-compat":
            raise Refused("REFUSED[FOREIGN_REPOSITORY]")
        if not re.fullmatch(r"[0-9a-f]{40}", self.sha):
            raise Refused("REFUSED[INEXACT_SUBJECT]")
        if not re.fullmatch(r"[0-9a-f]{40}", self.ggen_sha):
            raise Refused("REFUSED[INEXACT_GGEN_SUBJECT]")


@dataclass(frozen=True)
class TargetObservation:
    rule: str
    output_file: str
    absolute: bool
    traversal: bool
    contained: bool
    source_repo: str
    target_repo: str
    isolated_stage: bool
    failure_code: str | None
