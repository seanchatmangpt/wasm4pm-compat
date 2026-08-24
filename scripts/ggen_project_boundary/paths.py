from pathlib import PurePosixPath


def classify_path(output_file: str) -> tuple[bool, bool, bool]:
    normalized = output_file.replace("\\", "/")
    absolute = normalized.startswith("/") or (len(normalized) > 2 and normalized[1:3] == ":/")
    parts = PurePosixPath(normalized).parts
    traversal = ".." in parts
    contained = not absolute and not traversal
    return absolute, traversal, contained
