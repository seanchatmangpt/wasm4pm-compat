// Law: WorkflowPatternDistinctionLaw — PatternNet<Milestone> is a different type from PatternNet<Sequence>; named workflow patterns are distinct structural laws that cannot be confused at function boundaries (Russell, van der Aalst & ter Hofstede 2016)

// COMPILE-FAIL: PatternNet with Milestone pattern cannot be mistaken for Sequence
//
// Law: Russell, van der Aalst & ter Hofstede (2016) — each workflow pattern
// is a distinct structural law. A PatternNet claiming to model WCP-18 (Milestone)
// cannot be used where a PatternNet claiming to model WCP-1 (Sequence) is expected.
// This prevents pattern confusion at type boundaries.
#![feature(adt_const_params)]
#![allow(incomplete_features)]

use wasm4pm_compat::law::WorkflowPattern;

struct PatternNet<const P: WorkflowPattern>;

fn sequence_only(_: PatternNet<{ WorkflowPattern::Sequence }>) {}

fn main() {
    let milestone: PatternNet<{ WorkflowPattern::Milestone }> = PatternNet;
    // Milestone ≠ Sequence: must fail to compile with type mismatch
    sequence_only(milestone);
}
