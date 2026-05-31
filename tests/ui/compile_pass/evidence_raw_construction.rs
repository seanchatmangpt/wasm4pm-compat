// COMPILE-PASS: Evidence<&str, Raw, Ocel20> lawful construction via Evidence::raw
//
// Law: Raw is the only freely-constructible stage. Evidence::raw wraps an
// untrusted value as Raw evidence tagged with a named witness at zero cost.
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::Raw;
use wasm4pm_compat::witness::Ocel20;

fn main() {
    let raw: Evidence<&str, Raw, Ocel20> = Evidence::raw("untrusted-ocel-bytes");
    assert_eq!(raw.value, "untrusted-ocel-bytes");
}
