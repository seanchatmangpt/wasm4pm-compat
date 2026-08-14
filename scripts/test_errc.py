#!/usr/bin/env python3
import copy
import unittest

from scripts import errc


HEAD = "1" * 40
REPO = "seanchatmangpt/wasm4pm-compat"


def matrix(rows, falsifiers=None):
    tests = sorted(rows)
    falsifiers = falsifiers or sorted(next(iter(rows.values())))
    return {
        "schema": errc.SCHEMA_MATRIX,
        "tests": tests,
        "falsifiers": falsifiers,
        "observations": {t: rows[t] for t in tests},
    }


def representative_matrix():
    return matrix({
        "t-a": {"m-a": "KILL", "m-b": "SURVIVE", "m-c": "SURVIVE"},
        "t-b": {"m-a": "KILL", "m-b": "SURVIVE", "m-c": "SURVIVE"},
        "t-c": {"m-a": "SURVIVE", "m-b": "KILL", "m-c": "SURVIVE"},
        "t-d": {"m-a": "SURVIVE", "m-b": "SURVIVE", "m-c": "KILL"},
    })


class ERRCLaws(unittest.TestCase):
    def assert_refused(self, code, fn):
        with self.assertRaises(errc.ERRCRefusal) as caught:
            fn()
        self.assertEqual(caught.exception.code, code)

    def test_exact_compression_removes_only_redundancy(self):
        receipt = errc.build_receipt(representative_matrix(), REPO, HEAD)
        self.assertEqual(receipt["retained_tests"], ["t-a", "t-c", "t-d"])
        self.assertEqual(receipt["counts"]["tests_original"], 4)
        self.assertEqual(receipt["counts"]["tests_retained"], 3)
        self.assertIn(["t-a", "t-b"], receipt["structure"]["equivalent_tests"])
        self.assertEqual(set(receipt["witnesses"]), set(receipt["obligations"]))

    def test_independent_replay_is_deterministic(self):
        doc = representative_matrix()
        one = errc.build_receipt(doc, REPO, HEAD)
        two = errc.build_receipt(doc, REPO, HEAD)
        self.assertEqual(one, two)
        verdict = errc.verify_receipt(doc, one)
        self.assertEqual(verdict["status"], "VERIFIED")
        self.assertEqual(verdict["receipt_digest"], one["receipt_digest"])

    def test_lexicographic_tie_break(self):
        doc = matrix({
            "t-a": {"m-a": "KILL"},
            "t-b": {"m-a": "KILL"},
        })
        receipt = errc.build_receipt(doc, REPO, HEAD)
        self.assertEqual(receipt["retained_tests"], ["t-a"])

    def test_unknown_refuses_exact_theorem(self):
        doc = matrix({"t-a": {"m-a": "UNKNOWN"}})
        self.assert_refused(
            errc.RefusalCode.UNKNOWN_OBSERVATION,
            lambda: errc.build_receipt(doc, REPO, HEAD),
        )

    def test_invalid_head_refuses_subject_binding(self):
        self.assert_refused(
            errc.RefusalCode.INVALID_HEAD,
            lambda: errc.build_receipt(representative_matrix(), REPO, "main"),
        )

    def test_solver_envelope_refuses_instead_of_approximating(self):
        self.assert_refused(
            errc.RefusalCode.SOLVER_LIMIT_EXCEEDED,
            lambda: errc.build_receipt(representative_matrix(), REPO, HEAD, max_tests=3),
        )

    def test_zero_obligation_space_has_empty_exact_basis(self):
        doc = matrix({
            "t-a": {"m-a": "SURVIVE", "m-b": "SURVIVE"},
            "t-b": {"m-a": "SURVIVE", "m-b": "SURVIVE"},
        })
        receipt = errc.build_receipt(doc, REPO, HEAD)
        self.assertEqual(receipt["retained_tests"], [])
        self.assertEqual(receipt["obligations"], [])
        self.assertEqual(errc.verify_receipt(doc, receipt)["status"], "VERIFIED")

    def test_sabotaged_retained_set_is_detected(self):
        doc = representative_matrix()
        receipt = errc.build_receipt(doc, REPO, HEAD)
        receipt["retained_tests"] = ["t-a", "t-c"]
        self.assert_refused(
            errc.RefusalCode.RECEIPT_MISMATCH,
            lambda: errc.verify_receipt(doc, receipt),
        )

    def test_self_consistent_tampered_witness_is_detected_by_replay(self):
        doc = representative_matrix()
        receipt = errc.build_receipt(doc, REPO, HEAD)
        tampered = copy.deepcopy(receipt)
        key = next(iter(tampered["witnesses"]))
        tampered["witnesses"][key] = "t-b"
        tampered["witnesses_digest"] = errc.digest(tampered["witnesses"])
        tampered.pop("receipt_digest")
        tampered["receipt_digest"] = errc.digest(tampered)
        self.assert_refused(
            errc.RefusalCode.RECEIPT_MISMATCH,
            lambda: errc.verify_receipt(doc, tampered),
        )

    def test_matrix_tampering_is_detected(self):
        doc = representative_matrix()
        receipt = errc.build_receipt(doc, REPO, HEAD)
        altered = copy.deepcopy(doc)
        altered["observations"]["t-d"]["m-c"] = "SURVIVE"
        self.assert_refused(
            errc.RefusalCode.RECEIPT_MISMATCH,
            lambda: errc.verify_receipt(altered, receipt),
        )

    def test_noncanonical_ids_refuse_replay_ambiguity(self):
        doc = representative_matrix()
        doc["tests"] = list(reversed(doc["tests"]))
        self.assert_refused(
            errc.RefusalCode.INVALID_MATRIX,
            lambda: errc.build_receipt(doc, REPO, HEAD),
        )


if __name__ == "__main__":
    unittest.main()
