#!/usr/bin/env python3
"""Chicago-style tests for scripts/check_test_targets_compile.py.

Real collaborators only: each case writes a real throwaway cargo crate to a temp
dir and runs the real guard, which runs real ``cargo``. No test doubles.
Run: python3 -m unittest scripts.test_check_test_targets_compile
"""

import subprocess
import sys
import tempfile
import textwrap
import unittest
from pathlib import Path

GUARD = Path(__file__).resolve().parent / "check_test_targets_compile.py"

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


def make_crate(root: Path, tests: dict) -> Path:
    (root / "src").mkdir(parents=True)
    (root / "tests").mkdir()
    (root / "Cargo.toml").write_text(
        '[package]\nname = "fixture"\nversion = "0.0.0"\nedition = "2021"\n\n[workspace]\n'
    )
    (root / "src" / "lib.rs").write_text(LIB)
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


if __name__ == "__main__":
    unittest.main()
