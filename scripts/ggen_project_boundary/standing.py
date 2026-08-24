def standing(census: dict) -> str:
    if census["escaped_rules"]:
        return "BUILD_BROKEN"
    if census["missing_input_digests"]:
        return "UNKNOWN"
    if census["active_rules"] == 0:
        return "UNSUPPORTED"
    return "PARTIAL_ALIVE"
