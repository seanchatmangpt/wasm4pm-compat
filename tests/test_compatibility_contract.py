from __future__ import annotations

import copy
import importlib.util
import json
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location(
    "compatibility_contract", ROOT / "scripts/compatibility-contract.py"
)
assert SPEC is not None and SPEC.loader is not None
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class CompatibilityContractTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.contract = json.loads(
            (ROOT / "compatibility/contract-v1.json").read_text(encoding="utf-8")
        )

    def test_identical_contract_is_compatible(self) -> None:
        result = MODULE.classify_contract_change(self.contract, copy.deepcopy(self.contract))
        self.assertEqual(result["classification"], "compatible")
        self.assertEqual(result["breaking"], [])

    def test_feature_addition_is_breaking(self) -> None:
        changed = copy.deepcopy(self.contract)
        changed["features"]["public"].append("telemetry")
        result = MODULE.classify_contract_change(self.contract, changed)
        self.assertEqual(result["classification"], "breaking")
        self.assertIn(
            "PUBLIC_FEATURE_SET_CHANGED",
            {item["code"] for item in result["breaking"]},
        )

    def test_capability_owner_change_is_breaking(self) -> None:
        changed = copy.deepcopy(self.contract)
        changed["capabilities"][0]["owner"] = "wasm4pm"
        result = MODULE.classify_contract_change(self.contract, changed)
        self.assertIn(
            "CAPABILITY_OWNER_CHANGED",
            {item["code"] for item in result["breaking"]},
        )

    def test_required_module_addition_is_compatible(self) -> None:
        changed = copy.deepcopy(self.contract)
        changed["required_modules"].append("future_shape")
        result = MODULE.classify_contract_change(self.contract, changed)
        self.assertEqual(result["classification"], "compatible")
        self.assertIn(
            "REQUIRED_MODULE_ADDED",
            {item["code"] for item in result["compatible"]},
        )

    def test_package_version_change_is_informational(self) -> None:
        changed = copy.deepcopy(self.contract)
        changed["package"]["version"] = "26.6.30"
        result = MODULE.classify_contract_change(self.contract, changed)
        self.assertEqual(result["classification"], "compatible")
        self.assertIn(
            "PACKAGE_VERSION_CHANGED",
            {item["code"] for item in result["informational"]},
        )

    def test_relaxing_forbidden_symbol_is_breaking(self) -> None:
        changed = copy.deepcopy(self.contract)
        changed["forbidden_source_tokens"] = []
        result = MODULE.classify_contract_change(self.contract, changed)
        self.assertIn(
            "FORBIDDEN_TOKEN_RELAXED",
            {item["code"] for item in result["breaking"]},
        )

    def write_minimal_repository(self, root: Path) -> None:
        (root / "src/diagnostic").mkdir(parents=True)
        (root / "Cargo.toml").write_text(
            """[package]
name = "wasm4pm-compat"
version = "26.6.29"
edition = "2021"

[features]
default = ["formats"]
formats = []
strict = []
wasm4pm = []
""",
            encoding="utf-8",
        )
        (root / "rust-toolchain.toml").write_text(
            "[toolchain]\nchannel = \"nightly-2026-06-22\"\n",
            encoding="utf-8",
        )
        modules = "\n".join(
            f"pub mod {name};" for name in self.contract["required_modules"]
        )
        (root / "src/lib.rs").write_text(
            "//! structure-only compatibility court\n" + modules + "\n",
            encoding="utf-8",
        )
        exports = ", ".join(self.contract["required_prelude_exports"])
        (root / "src/prelude.rs").write_text(
            f"pub use crate::surface::{{{exports}}};\n", encoding="utf-8"
        )
        codes = ",".join(
            json.dumps(item["code"]) for item in self.contract["capabilities"]
        )
        (root / "src/diagnostic/doctor.rs").write_text(
            "pub enum DoctorStanding { Unknown, PartialAlive, Blocked }\n"
            "pub enum RouteTarget { Compat, Wasm4pm, ExternalVerifier }\n"
            "fn route() { let _ = RouteTarget::Wasm4pm; "
            "let _ = RouteTarget::ExternalVerifier; "
            "let _ = DoctorStanding::PartialAlive; }\n"
            f"const CODES: &[&str] = &[{codes}];\n",
            encoding="utf-8",
        )

    def test_static_check_accepts_contract_closed_repository(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            self.write_minimal_repository(root)
            contract_path = root / "contract.json"
            contract_path.write_text(json.dumps(self.contract), encoding="utf-8")
            result = MODULE.check_contract(root, contract_path)
            self.assertEqual(result["standing"], "PARTIAL_ALIVE")
            self.assertEqual(result["failure_count"], 0)

    def test_static_check_refuses_forbidden_engine_symbol(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            self.write_minimal_repository(root)
            contract_path = root / "contract.json"
            contract_path.write_text(json.dumps(self.contract), encoding="utf-8")
            with (root / "src/lib.rs").open("a", encoding="utf-8") as handle:
                handle.write("// bcinr_engine must never return\n")
            result = MODULE.check_contract(root, contract_path)
            self.assertEqual(result["standing"], "BLOCKED")
            self.assertIn(
                "FORBIDDEN_SOURCE_TOKEN",
                {item["code"] for item in result["failures"]},
            )


if __name__ == "__main__":
    unittest.main()
