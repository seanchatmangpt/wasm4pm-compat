import unittest
from scripts.ggen_project_boundary.target import observe_target

class T(unittest.TestCase):
    def test_cross_repo_requires_isolation(self):
        bad = observe_target("x", "generated/x.rs", "o/a", "o/b", False)
        self.assertEqual(bad.failure_code, "FM-WRITE-002")
        good = observe_target("x", "generated/x.rs", "o/a", "o/b", True)
        self.assertIsNone(good.failure_code)
