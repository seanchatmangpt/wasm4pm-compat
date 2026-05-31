// COMPILE-PASS: Evidence<&str, Parsed, Xes1849> lawful construction via into_parsed
//
// Law: Raw → Parsed is the first lifecycle step. into_parsed() advances a
// well-formed raw value to Parsed without admission — the value is now
// structurally decoded but not yet judged against any named authority.
use wasm4pm_compat::evidence::Evidence;
use wasm4pm_compat::state::Parsed;
use wasm4pm_compat::witness::Xes1849;

fn main() {
    let raw = Evidence::<_, _, Xes1849>::raw("well-formed-xes-bytes");
    let parsed: Evidence<&str, Parsed, Xes1849> = raw.into_parsed();
    assert_eq!(parsed.value, "well-formed-xes-bytes");
}
