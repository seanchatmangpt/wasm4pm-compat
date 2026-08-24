import unittest
from scripts.ggen_project_boundary.receipt import manufacture_receipt
from scripts.ggen_project_boundary.replay import replay_receipt
from scripts.ggen_project_boundary.types import Refused

class T(unittest.TestCase):
    def test_tamper_refuses(self):
        obs={"subject":{"repo":"seanchatmangpt/wasm4pm-compat","sha":"a"*40,"ggen_sha":"b"*40},"manifest":"ggen/ggen.toml","output_dir":".","rules":[],"disabled_cross_repo_targets":(),"authority":"OBSERVE|VERIFY","actuation_performed":False}
        receipt=manufacture_receipt(obs)
        self.assertEqual(replay_receipt(receipt),"REPLAY_MATCH")
        receipt["body"]["standing"]="ALIVE"
        with self.assertRaises(Refused): replay_receipt(receipt)
