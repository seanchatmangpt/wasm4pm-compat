// Law: XesStandardPrefixAllFourLaw — XesStandardPrefix names all seven IEEE 1849-2016 standard prefixes (concept, time, lifecycle, org, cost, identity, semantic); as_str()/parse() round-trip and all() returns exactly seven
// COMPILE-PASS: xes-standard-prefix-all-four — proves XesStandardPrefix names
// all seven IEEE 1849-2016 standard prefixes (concept, time, lifecycle, org, cost, identity, semantic),
// that as_str() returns the canonical string, parse() round-trips correctly,
// and all() returns exactly seven variants.
use wasm4pm_compat::xes::XesStandardPrefix;

fn main() {
    assert_eq!(XesStandardPrefix::Concept.as_str(), "concept");
    assert_eq!(XesStandardPrefix::Time.as_str(), "time");
    assert_eq!(XesStandardPrefix::Lifecycle.as_str(), "lifecycle");
    assert_eq!(XesStandardPrefix::Org.as_str(), "org");
    assert_eq!(XesStandardPrefix::Cost.as_str(), "cost");
    assert_eq!(XesStandardPrefix::Identity.as_str(), "identity");
    assert_eq!(XesStandardPrefix::Semantic.as_str(), "semantic");

    // parse() round-trips.
    assert_eq!(XesStandardPrefix::parse("concept"), Some(XesStandardPrefix::Concept));
    assert_eq!(XesStandardPrefix::parse("time"), Some(XesStandardPrefix::Time));
    assert_eq!(XesStandardPrefix::parse("lifecycle"), Some(XesStandardPrefix::Lifecycle));
    assert_eq!(XesStandardPrefix::parse("org"), Some(XesStandardPrefix::Org));

    // Non-standard returns None.
    assert_eq!(XesStandardPrefix::parse("custom"), None);
    assert_eq!(XesStandardPrefix::parse(""), None);

    // all() returns exactly seven entries.
    assert_eq!(XesStandardPrefix::all().len(), 7);

    // Display delegates to as_str().
    assert_eq!(format!("{}", XesStandardPrefix::Lifecycle), "lifecycle");
}
