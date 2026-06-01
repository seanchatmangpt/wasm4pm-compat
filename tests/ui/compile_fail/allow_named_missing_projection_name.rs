// COMPILE-FAIL: AllowNamedProjection path without a ProjectionName in the report
//
// Law: loss-allow-named-requires-projection-name-field
//
// Under LossPolicy::AllowNamedProjection the projection must carry a
// ProjectionName — an explicit, stable identifier for the projection. A
// LossReport is constructed with (ProjectionName, LossPolicy, Items). Passing a
// bare u32 (or any non-ProjectionName type) in the first position is rejected:
// the type system requires ProjectionName, not an arbitrary identifier.
//
// This fixture attempts to construct a LossReport for an AllowNamedProjection
// path by placing a u32 counter in the ProjectionName position. This violates the
// law: the name field must carry a typed ProjectionName, not an untyped integer.
// Without a typed name, the projection cannot be audited by identity.
//
// Expected error: mismatched types — u32 (or integer) is not ProjectionName.
use wasm4pm_compat::loss::{LossPolicy, LossReport};

enum OcelShape {}
enum XesShape {}

fn main() {
    // VIOLATION: 42_u32 is not a ProjectionName. The AllowNamedProjection law
    // requires the first argument to LossReport::new to be a ProjectionName —
    // a typed newtype wrapper, not a bare integer.
    // Without a named ProjectionName the projection cannot be audited by identity.
    let _report = LossReport::<OcelShape, XesShape, Vec<String>>::new(
        42_u32,
        LossPolicy::AllowNamedProjection,
        vec!["dropped: item-type".to_string()],
    );
}
