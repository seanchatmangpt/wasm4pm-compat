// COMPILE-FAIL: Format export law — FormatExport cannot be passed where LossyFormatExport is required.
// Law: FormatExport and LossyFormatExport are distinct types. A lossless export
// does not carry a LossReport and cannot be used as a lossy export that requires one.
use wasm4pm_compat::formats::{FormatExport, FormatKind, LossyFormatExport};

fn requires_lossy_export(_e: LossyFormatExport) {}

fn main() {
    let lossless = FormatExport::lossless(FormatKind::OcelJson, vec![]);
    // This must fail: FormatExport is not LossyFormatExport.
    requires_lossy_export(lossless);
}
