// COMPILE-PASS: NamedLoss — proves the named-loss descriptor pairs a
// ProjectionName with a loss-category label and is usable as LossReport::Lost.
//
// Proves that:
//   1. NamedLoss is constructible from a ProjectionName + category &'static str.
//   2. projection() and category() accessors return the expected values.
//   3. NamedLoss implements Display as "<projection>/<category>".
//   4. NamedLoss can be the Items type of a LossReport (category-only loss record).
//   5. LossReport::summary() produces the expected NamedLoss from a report.

use wasm4pm_compat::loss::{LossPolicy, LossReport, NamedLoss, ProjectionName};

enum OcelShape {}
enum XesShape {}

fn main() {
    // 1. Construction and accessor round-trip.
    let loss = NamedLoss::new(
        ProjectionName("ocel-flatten-to-xes:by-order"),
        "DroppedObjectTypeLinks",
    );
    assert_eq!(loss.projection().as_str(), "ocel-flatten-to-xes:by-order");
    assert_eq!(loss.category(), "DroppedObjectTypeLinks");

    // 2. Display: "<projection>/<category>".
    assert_eq!(
        format!("{}", loss),
        "ocel-flatten-to-xes:by-order/DroppedObjectTypeLinks",
    );

    // 3. NamedLoss as the Items type of a LossReport (category-only record).
    let report = LossReport::<OcelShape, XesShape, NamedLoss>::new(
        ProjectionName("ocel-flatten-to-xes:by-order"),
        LossPolicy::AllowNamedProjection,
        NamedLoss::new(
            ProjectionName("ocel-flatten-to-xes:by-order"),
            "FlattenedMultiObjectRelation",
        ),
    );
    assert_eq!(report.policy, LossPolicy::AllowNamedProjection);
    assert_eq!(report.lost.category(), "FlattenedMultiObjectRelation");

    // 4. LossReport::summary() produces a NamedLoss from a Vec-based report.
    let vec_report = LossReport::<OcelShape, XesShape, Vec<&str>>::new(
        ProjectionName("ocel-flatten-to-xes:by-order"),
        LossPolicy::AllowLossWithReport,
        vec!["item", "invoice"],
    );
    let summary = vec_report.summary("DroppedObjectTypeLinks");
    assert_eq!(summary.projection().as_str(), "ocel-flatten-to-xes:by-order");
    assert_eq!(summary.category(), "DroppedObjectTypeLinks");

    // 5. Copy semantics — NamedLoss is Copy.
    let a = NamedLoss::new(ProjectionName("p"), "SomeLoss");
    let b = a; // copy
    assert_eq!(a.category(), b.category());
}
