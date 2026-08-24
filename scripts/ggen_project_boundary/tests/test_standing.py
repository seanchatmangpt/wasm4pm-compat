import unittest
from scripts.ggen_project_boundary.standing import standing

class T(unittest.TestCase):
    def test_failure_dominates(self):
        self.assertEqual(standing({"escaped_rules":1,"missing_input_digests":0,"active_rules":5}), "BUILD_BROKEN")
        self.assertEqual(standing({"escaped_rules":0,"missing_input_digests":1,"active_rules":5}), "UNKNOWN")
        self.assertEqual(standing({"escaped_rules":0,"missing_input_digests":0,"active_rules":5}), "PARTIAL_ALIVE")
