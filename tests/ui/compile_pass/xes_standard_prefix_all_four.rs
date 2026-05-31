// COMPILE-PASS: xes-standard-prefix-all-four — proves XesStandardPrefix names
// all four IEEE 1849-2016 standard prefixes (concept, time, lifecycle, org),
// that as_str() returns the canonical string, parse() round-trips correctly,
// and all() returns exactly four variants.
use wasm4pm_compat::xes::XesStandardPrefix;

fn main() {
    assert_eq!(XesStandardPrefix::Concept.as_str(), "concept");
    assert_eq!(XesStandardPrefix::Time.as_str(), "time");
    assert_eq!(XesStandardPrefix::Lifecycle.as_str(), "lifecycle");
    assert_eq!(XesStandardPrefix::Org.as_str(), "org");

    // parse() round-trips.
    assert_eq!(XesStandardPrefix::parse("concept"), Some(XesStandardPrefix::Concept));
    assert_eq!(XesStandardPrefix::parse("time"), Some(XesStandardPrefix::Time));
    assert_eq!(XesStandardPrefix::parse("lifecycle"), Some(XesStandardPrefix::Lifecycle));
    assert_eq!(XesStandardPrefix::parse("org"), Some(XesStandardPrefix::Org));

    // Non-standard returns None.
    assert_eq!(XesStandardPrefix::parse("custom"), None);
    assert_eq!(XesStandardPrefix::parse(""), None);

    // all() returns exactly four entries.
    assert_eq!(XesStandardPrefix::all().len(), 4);

    // Display delegates to as_str().
    assert_eq!(format!("{}", XesStandardPrefix::Lifecycle), "lifecycle");
}
