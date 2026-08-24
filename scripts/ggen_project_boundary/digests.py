from hashlib import sha256
from pathlib import Path


def file_digest(root: Path, relative: str) -> str:
    path = root / relative
    if not path.is_file():
        return "MISSING"
    return sha256(path.read_bytes()).hexdigest()
