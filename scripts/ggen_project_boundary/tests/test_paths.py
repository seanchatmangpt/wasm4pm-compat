import unittest
from scripts.ggen_project_boundary.paths import classify_path

class T(unittest.TestCase):
    def test_relative_absolute_traversal(self):
        self.assertEqual(classify_path("src/x.rs"), (False, False, True))
        self.assertEqual(classify_path("../wasm4pm/x.rs"), (False, True, False))
        self.assertEqual(classify_path("/tmp/x.rs"), (True, False, False))
