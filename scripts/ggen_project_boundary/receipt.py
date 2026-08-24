from hashlib import sha256
import json

from .census import census
from .standing import standing


def manufacture_receipt(observation: dict) -> dict:
    counts = census(observation)
    body = {
        "schema": "wasm4pm-compat.ggen-project-boundary/1",
        "subject": observation["subject"],
        "manifest": observation["manifest"],
        "output_dir": observation["output_dir"],
        "census": counts,
        "standing": standing(counts),
        "authority": "OBSERVE|VERIFY",
        "actuation_performed": False,
    }
    raw = json.dumps(body, sort_keys=True, separators=(",", ":"))
    return {"body": body, "sha256": sha256(raw.encode()).hexdigest()}
