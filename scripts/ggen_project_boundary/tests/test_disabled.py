import unittest
from scripts.ggen_project_boundary.disabled import disabled_cross_repo_targets

class T(unittest.TestCase):
    def test_disabled_target(self):
        text = '# output_file = "../wasm4pm/src/x.rs"\noutput_file = "src/y.rs"\n'
        self.assertEqual(disabled_cross_repo_targets(text), ("../wasm4pm/src/x.rs",))
