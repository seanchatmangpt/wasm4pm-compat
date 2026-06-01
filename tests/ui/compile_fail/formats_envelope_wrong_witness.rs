// COMPILE-FAIL: Format witness law — FormatEnvelope<Ocel20> cannot be passed
// where FormatEnvelope<Xes1849> is required.
// Law: The witness type parameter W in FormatEnvelope<W> names the authority;
// OCEL 2.0 and XES 1849 are different formats and their envelopes are distinct types.
use wasm4pm_compat::formats::{FormatEnvelope, FormatKind};
use wasm4pm_compat::witness::{Ocel20, Xes1849};

fn requires_xes_envelope(_e: FormatEnvelope<Xes1849>) {}

fn main() {
    let ocel_env: FormatEnvelope<Ocel20> = FormatEnvelope::new(FormatKind::OcelJson, vec![]);
    // This must fail: FormatEnvelope<Ocel20> is not FormatEnvelope<Xes1849>.
    requires_xes_envelope(ocel_env);
}
