// COMPILE-FAIL: Witness confusion law — Evidence<T, Admitted, DeclareFamily> cannot be used
// where Evidence<T, Admitted, OcpqPaper> is required.
// Law: DeclareFamily and OcpqPaper name different authorities; their witness tags
// produce incompatible Evidence types.
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::Admitted;
use wasm4pm_compat::witness::{DeclareFamily, OcpqPaper};

fn requires_ocpq_evidence(_: Evidence<String, Admitted, OcpqPaper>) {}

fn main() {
    let decl_ev: Evidence<String, Admitted, DeclareFamily> = todo!();
    // This must fail: DeclareFamily witness is not OcpqPaper.
    requires_ocpq_evidence(decl_ev);
}
