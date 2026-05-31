// COMPILE-PASS: Wasm4pmBridge witness metadata — proves Witness trait constants for internal bridge

use wasm4pm_compat::witness::{Wasm4pmBridge, Witness, WitnessFamily};

fn main() {
    assert_eq!(Wasm4pmBridge::KEY, "wasm4pm-bridge");
    assert_eq!(Wasm4pmBridge::YEAR, None);
    assert_eq!(Wasm4pmBridge::FAMILY, WitnessFamily::InternalBridge);
}
