#!/usr/bin/env python3
"""Chicago-style tests for scripts/check_test_targets_compile.py.

Real collaborators only: each case writes a real throwaway cargo crate to a temp
dir and runs the real guard, which runs real ``cargo``. No test doubles.
Run: python3 -m unittest scripts.test_check_test_targets_compile
"""

import importlib.util
import json
import subprocess
import sys
import tempfile
import textwrap
import time
import unittest
from pathlib import Path

GUARD = Path(__file__).resolve().parent / "check_test_targets_compile.py"
_SPEC = importlib.util.spec_from_file_location("check_test_targets_compile", GUARD)
guard = importlib.util.module_from_spec(_SPEC)
_SPEC.loader.exec_module(guard)

LIB = textwrap.dedent(
    """
    pub mod prelude {
        pub mod protocol {
            pub struct Intent;
            impl Intent { pub fn ok() -> bool { true } }
        }
        pub use protocol::Intent as ProtocolIntent;
    }
    """
)

GOOD = textwrap.dedent(
    """
    use fixture::prelude::protocol::Intent;
    #[test]
    fn good() { assert!(Intent::ok()); }
    """
)

# The exact defect class: the bare name is only re-exported under an alias.
BARE_NAME = textwrap.dedent(
    """
    use fixture::prelude::*;
    #[test]
    fn bare() { assert!(Intent::ok()); }
    """
)

# The exact defect class: importing a module path that is not at the crate root.
WRONG_PATH = textwrap.dedent(
    """
    use fixture::protocol::Intent;
    #[test]
    fn wrong() { assert!(Intent::ok()); }
    """
)


def make_crate(root: Path, tests: dict, manifest_extra: str = "", lib: str = LIB) -> Path:
    (root / "src").mkdir(parents=True)
    (root / "tests").mkdir()
    (root / "Cargo.toml").write_text(
        '[package]\nname = "fixture"\nversion = "0.0.0"\nedition = "2021"\n'
        + manifest_extra
        + "\n[workspace]\n"
    )
    (root / "src" / "lib.rs").write_text(lib)
    for name, body in tests.items():
        (root / "tests" / f"{name}.rs").write_text(body)
    return root / "Cargo.toml"


def run_guard(manifest: Path):
    return subprocess.run(
        [sys.executable, str(GUARD), "--manifest-path", str(manifest)],
        capture_output=True,
        text=True,
    )


class CompileClosureGuard(unittest.TestCase):
    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.root = Path(self._tmp.name)

    def tearDown(self):
        self._tmp.cleanup()

    def test_all_targets_compile_admits(self):
        manifest = make_crate(self.root / "c", {"a_good": GOOD, "b_good": GOOD})
        proc = run_guard(manifest)
        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertIn('"missing": []', proc.stdout)
        self.assertIn('"compiled": 2', proc.stdout)

    def test_alias_only_name_is_refused_by_target_name(self):
        manifest = make_crate(self.root / "c", {"a_good": GOOD, "ws4_bare": BARE_NAME})
        proc = run_guard(manifest)
        self.assertEqual(proc.returncode, 1, proc.stdout + proc.stderr)
        self.assertIn('"missing": ["ws4_bare"]', proc.stdout)
        self.assertIn("FAIL ws4_bare", proc.stderr)
        self.assertIn("REFUSED(TEST_TARGET_COMPILE_CLOSURE): 1 of 2", proc.stderr)

    def test_every_broken_target_is_named_not_just_the_first(self):
        manifest = make_crate(
            self.root / "c",
            {"a_good": GOOD, "ws4_bare": BARE_NAME, "ws4_wrong_path": WRONG_PATH},
        )
        proc = run_guard(manifest)
        self.assertEqual(proc.returncode, 1, proc.stdout + proc.stderr)
        self.assertIn('"missing": ["ws4_bare", "ws4_wrong_path"]', proc.stdout)
        self.assertIn("E0432", proc.stderr)
        self.assertIn("REFUSED(TEST_TARGET_COMPILE_CLOSURE): 2 of 3", proc.stderr)

    def test_crate_without_test_files_is_refused(self):
        manifest = make_crate(self.root / "c", {})
        proc = run_guard(manifest)
        self.assertEqual(proc.returncode, 1)
        self.assertIn("REFUSED(NO_TEST_TARGETS)", proc.stderr)


class CompileClosureGuardBoundaries(unittest.TestCase):
    """Boundary / adversarial cases added by the v26.9.26 harden pass (PR #35).

    Each case runs the real guard against a real cargo crate, except the
    parse_stream cases, which feed the real parser real JSON lines.
    """

    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.root = Path(self._tmp.name)

    def tearDown(self):
        self._tmp.cleanup()

    def test_explicit_test_target_with_other_name_is_admitted(self):
        # Falsifier found in harden pass: name-keyed matching refused a file
        # registered as [[test]] name = "renamed", path = "tests/orig.rs".
        manifest = make_crate(
            self.root / "c",
            {"orig": GOOD, "other": GOOD},
            manifest_extra='\n[[test]]\nname = "renamed"\npath = "tests/orig.rs"\n',
        )
        proc = run_guard(manifest)
        self.assertEqual(proc.returncode, 0, proc.stdout + proc.stderr)
        self.assertIn('"compiled": 2', proc.stdout)

    def test_same_name_target_elsewhere_does_not_admit_unregistered_file(self):
        # Falsifier found in harden pass: with autotests off, a [[test]] named
        # "orig" pointing elsewhere made name-keyed matching admit tests/orig.rs,
        # which is not a registered target at all.
        crate = self.root / "c"
        manifest = make_crate(
            crate,
            {"orig": GOOD},
            manifest_extra='autotests = false\n\n[[test]]\nname = "orig"\npath = "elsewhere/orig.rs"\n',
        )
        (crate / "elsewhere").mkdir()
        (crate / "elsewhere" / "orig.rs").write_text(GOOD)
        proc = run_guard(manifest)
        self.assertEqual(proc.returncode, 1, proc.stdout + proc.stderr)
        self.assertIn('"missing": ["orig"]', proc.stdout)

    def test_unregistered_file_with_autotests_off_is_refused(self):
        manifest = make_crate(
            self.root / "c",
            {"a_good": GOOD, "b_orphan": GOOD},
            manifest_extra='autotests = false\n\n[[test]]\nname = "a_good"\npath = "tests/a_good.rs"\n',
        )
        proc = run_guard(manifest)
        self.assertEqual(proc.returncode, 1, proc.stdout + proc.stderr)
        self.assertIn('"missing": ["b_orphan"]', proc.stdout)

    def test_broken_library_names_the_upstream_cause(self):
        manifest = make_crate(
            self.root / "c", {"a_good": GOOD, "b_good": GOOD}, lib="pub fn broken() -> u8 { \"x\" }\n"
        )
        proc = run_guard(manifest)
        self.assertEqual(proc.returncode, 1, proc.stdout + proc.stderr)
        self.assertIn('"missing": ["a_good", "b_good"]', proc.stdout)
        self.assertIn("blocked upstream by lib:fixture", proc.stderr)
        self.assertIn("E0308", proc.stderr)

    def test_unparseable_manifest_is_blocked_not_admitted(self):
        crate = self.root / "c"
        manifest = make_crate(crate, {"a_good": GOOD})
        manifest.write_text("[package\nname = ")
        proc = run_guard(manifest)
        self.assertEqual(proc.returncode, 2, proc.stdout + proc.stderr)
        self.assertIn("BLOCKED(CARGO_NO_STREAM)", proc.stderr)

    def test_warm_cache_does_not_hide_a_newly_broken_target(self):
        # Stale-subject falsifier: admit on a warm build, then break one file;
        # the cached artifact of the old subject must not keep it admitted.
        crate = self.root / "c"
        manifest = make_crate(crate, {"a_good": GOOD, "b_flip": GOOD})
        first = run_guard(manifest)
        self.assertEqual(first.returncode, 0, first.stderr)
        (crate / "tests" / "b_flip.rs").write_text(WRONG_PATH)
        second = run_guard(manifest)
        self.assertEqual(second.returncode, 1, second.stdout + second.stderr)
        self.assertIn('"missing": ["b_flip"]', second.stdout)
        (crate / "tests" / "b_flip.rs").write_text(GOOD)
        third = run_guard(manifest)
        self.assertEqual(third.returncode, 0, third.stderr)


def _artifact(name, src, kind="test", test=True):
    return json.dumps(
        {
            "reason": "compiler-artifact",
            "target": {"kind": [kind], "name": name, "src_path": src},
            "profile": {"test": test},
        }
    )


def _error(name, src, kind="test", code="E0432", text="unresolved import"):
    return json.dumps(
        {
            "reason": "compiler-message",
            "target": {"kind": [kind], "name": name, "src_path": src},
            "message": {"level": "error", "code": {"code": code}, "message": text},
        }
    )


class ParseStreamAdversarial(unittest.TestCase):
    def test_noise_malformed_and_non_object_lines_are_ignored(self):
        lines = [
            "   Compiling fixture v0.0.0",
            "{not json",
            "[1, 2, 3]",
            '"a string"',
            '{"reason": "compiler-artifact", "target": "not-a-dict"}',
            '{"reason": "compiler-artifact", "target": {"kind": ["test"], "name": "x"}}',
            _artifact("ok", "/r/tests/ok.rs"),
        ]
        compiled, errors, upstream = guard.parse_stream(lines)
        self.assertEqual(list(compiled.values()), ["ok"])
        self.assertEqual(errors, {})
        self.assertEqual(upstream, {})

    def test_duplicated_and_reordered_messages_are_idempotent(self):
        a = _artifact("ok", "/r/tests/ok.rs")
        e1 = _error("bad", "/r/tests/bad.rs", code="E0432", text="first")
        e2 = _error("bad", "/r/tests/bad.rs", code="E0433", text="second")
        forward = guard.parse_stream([a, e1, e2])
        shuffled = guard.parse_stream([e1, a, e2, a, e1, a])
        self.assertEqual(forward, shuffled)
        self.assertEqual(list(forward[1].values()), ["E0432 first"])

    def test_non_test_profile_artifact_does_not_count_as_compiled(self):
        compiled, _, _ = guard.parse_stream([_artifact("ok", "/r/tests/ok.rs", test=False)])
        self.assertEqual(compiled, {})

    def test_bin_or_lib_artifact_with_test_profile_is_not_a_test_target(self):
        compiled, _, _ = guard.parse_stream([_artifact("fixture", "/r/src/lib.rs", kind="lib")])
        self.assertEqual(compiled, {})

    def test_library_error_is_upstream_not_attributed_to_a_test(self):
        _, errors, upstream = guard.parse_stream([_error("fixture", "/r/src/lib.rs", kind="lib")])
        self.assertEqual(errors, {})
        self.assertEqual(upstream, {"lib:fixture": "E0432 unresolved import"})

    def test_parse_throughput_regression_bound(self):
        # Regression bound committed with the benchmark receipt
        # receipts/bench/check_test_targets_compile.json: 20k test targets
        # (100x this crate's 187) plus 20k dependency lines parse in < 2.0 s.
        lines = []
        for i in range(20_000):
            lines.append(_artifact(f"dep{i}", f"/r/deps/{i}.rs", kind="lib", test=False))
            lines.append(_artifact(f"t{i}", f"/r/tests/t{i}.rs"))
        start = time.perf_counter()
        compiled, _, _ = guard.parse_stream(lines)
        elapsed = time.perf_counter() - start
        self.assertEqual(len(compiled), 20_000)
        self.assertLess(elapsed, 2.0, f"parse_stream took {elapsed:.3f}s for 40k lines")


if __name__ == "__main__":
    unittest.main()
