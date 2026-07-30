#!/usr/bin/env python3

from __future__ import annotations

import unittest

from derive_status import StatusRefusal, derive_status, validate_claim


class DeriveStatusTests(unittest.TestCase):
    def test_ladder_and_build_failure(self) -> None:
        cases = [
            ({}, "UNKNOWN"),
            ({"declared": True}, "DECLARED"),
            ({"declared": True, "lean_build": True, "sorry_ax": True}, "BUILD_BROKEN"),
            (
                {
                    "declared": True,
                    "lean_build": True,
                    "sorry_ax": False,
                },
                "STATED",
            ),
            (
                {
                    "declared": True,
                    "lean_build": True,
                    "sorry_ax": False,
                    "aeneas_extracted": True,
                },
                "EXTRACTED",
            ),
            (
                {
                    "declared": True,
                    "lean_build": True,
                    "sorry_ax": False,
                    "aeneas_extracted": True,
                    "source_hash_matches": True,
                },
                "PROVEN",
            ),
        ]
        for evidence, expected in cases:
            with self.subTest(expected=expected):
                self.assertEqual(derive_status(evidence), expected)

    def test_poisoned_proven_claim_is_refused(self) -> None:
        evidence = {
            "declared": True,
            "lean_build": True,
            "sorry_ax": False,
            "aeneas_extracted": False,
            "source_hash_matches": False,
            "claimed_status": "PROVEN",
        }
        with self.assertRaisesRegex(StatusRefusal, "STATED_PROMOTED_TO_PROVEN"):
            validate_claim(evidence)

    def test_sorry_axiom_cannot_claim_stated(self) -> None:
        evidence = {
            "declared": True,
            "lean_build": True,
            "sorry_ax": True,
            "claimed_status": "STATED",
        }
        with self.assertRaisesRegex(StatusRefusal, "STATUS_MISMATCH:STATED:BUILD_BROKEN"):
            validate_claim(evidence)


if __name__ == "__main__":
    unittest.main()
