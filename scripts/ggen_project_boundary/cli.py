import argparse
import json
from pathlib import Path

from .manifest import inspect_manifest
from .rdf import to_turtle
from .receipt import manufacture_receipt
from .replay import replay_receipt
from .types import Subject


def main() -> int:
    p = argparse.ArgumentParser()
    p.add_argument("--root", default=".")
    p.add_argument("--sha", required=True)
    p.add_argument("--ggen-sha", required=True)
    p.add_argument("--format", choices=("json", "ttl"), default="json")
    args = p.parse_args()
    observation = inspect_manifest(Path(args.root), Subject("seanchatmangpt/wasm4pm-compat", args.sha, args.ggen_sha))
    receipt = manufacture_receipt(observation)
    assert replay_receipt(receipt) == "REPLAY_MATCH"
    if args.format == "ttl":
        print(to_turtle(observation), end="")
    else:
        print(json.dumps({"observation": observation, "receipt": receipt}, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
