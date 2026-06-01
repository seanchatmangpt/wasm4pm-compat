// Law: SummaryShapeVariantsLaw — all SummaryShape variants (Counts, TraceVariants, etc.) are constructible; the closed set is exhaustive
// COMPILE-PASS: SummaryShape all variants — proves all SummaryShape variants construct

use wasm4pm_compat::interop::SummaryShape;

fn main() {
    let shapes = [
        SummaryShape::Counts,
        SummaryShape::TraceVariants,
        SummaryShape::ActivityDistribution,
        SummaryShape::TimingProfile,
        SummaryShape::ObjectTypeDistribution,
    ];
    assert_eq!(shapes.len(), 5);
}
