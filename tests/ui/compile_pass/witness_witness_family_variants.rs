// COMPILE-PASS: WitnessFamily variants — proves all witness family enum variants construct

use wasm4pm_compat::witness::WitnessFamily;

fn main() {
    let families = [
        WitnessFamily::Standard,
        WitnessFamily::Paper,
        WitnessFamily::ApiGrammar,
        WitnessFamily::RustLaw,
        WitnessFamily::InternalBridge,
    ];
    assert_eq!(families.len(), 5);
    assert_eq!(families[0], WitnessFamily::Standard);
    assert_ne!(families[0], families[1]);
}
