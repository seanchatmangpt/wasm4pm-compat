import unittest
from scripts.ggen_project_boundary.types import Subject, Refused

class T(unittest.TestCase):
    def test_exact_subject(self):
        self.assertEqual(Subject("seanchatmangpt/wasm4pm-compat", "a"*40, "b"*40).sha, "a"*40)
        with self.assertRaises(Refused): Subject("seanchatmangpt/wasm4pm-compat", "bad", "b"*40)
