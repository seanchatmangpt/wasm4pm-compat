// COMPILE-PASS: Refusal::new and into_reason — proves refusal carries named law and yields it

use wasm4pm_compat::admission::Refusal;
use wasm4pm_compat::witness::{Ocel20, WfNetSoundnessPaper};

fn main() {
    let r = Refusal::<_, Ocel20>::new("DanglingEventObjectLink");
    assert_eq!(r.reason, "DanglingEventObjectLink");

    let r2 = Refusal::<_, WfNetSoundnessPaper>::new("UnsoundWfNet");
    let reason = r2.into_reason();
    assert_eq!(reason, "UnsoundWfNet");
}
