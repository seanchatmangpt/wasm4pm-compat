// Law: Wasm4pmBridgeWitnessMetadataLaw — Wasm4pmBridge implements Witness with KEY="wasm4pm-bridge", YEAR=None, FAMILY=InternalBridge; distinct from all external witness families
// COMPILE-PASS: Wasm4pmBridge witness metadata — proves Witness trait constants for internal bridge

use wasm4pm_compat::witness::{Wasm4pmBridge, Witness, WitnessFamily};

fn main() {
    assert_eq!(Wasm4pmBridge::KEY, "wasm4pm-bridge");
    assert_eq!(Wasm4pmBridge::YEAR, None);
    assert_eq!(Wasm4pmBridge::FAMILY, WitnessFamily::InternalBridge);
}
