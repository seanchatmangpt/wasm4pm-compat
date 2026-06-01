// COMPILE-FAIL: NamedLoss category cannot be dynamically constructed.
//
// Law: projection-loss-category-static
//
// NamedLoss(ProjectionName, &'static str) requires that the loss category label
// (e.g., "DroppedObjectTypeLinks") is a static, compile-time constant. Allowing
// dynamic categories would permit silent loss without explicit auditing — the
// exact category of loss might differ between runs.
//
// This fixture attempts to create a NamedLoss with a dynamic category (owned
// String). The type system must reject this because the category parameter is
// &'static str, not &str or String.
//
// Expected error: lifetime mismatch — String (or non-static &str) cannot be
// coerced to &'static str for the category parameter.

use wasm4pm_compat::loss::{NamedLoss, ProjectionName};

fn main() {
    let category = format!("LostItems_{}", 42);
    // This must fail: owned String is not &'static str.
    let _loss = NamedLoss::new(
        ProjectionName("some-projection"),
        category.as_str(),
    );
}
