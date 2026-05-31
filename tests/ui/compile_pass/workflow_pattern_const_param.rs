#![feature(adt_const_params)]
#![allow(incomplete_features)]

// COMPILE-PASS: WorkflowPattern ConstParamTy — named pattern as const generic
// param is a distinct type-level claim.
//
// Law: Russell, van der Aalst & ter Hofstede (2016) Workflow Patterns §2 —
// control-flow patterns are named structural laws; a type claiming to model
// WCP-2 (ParallelSplit) cannot be confused with WCP-4 (ExclusiveChoice).
use wasm4pm_compat::law::WorkflowPattern;

struct PatternNet<const P: WorkflowPattern>;

fn parallel_split_only(_: PatternNet<{ WorkflowPattern::ParallelSplit }>) {}
fn exclusive_choice_only(_: PatternNet<{ WorkflowPattern::ExclusiveChoice }>) {}

fn main() {
    let ps: PatternNet<{ WorkflowPattern::ParallelSplit }> = PatternNet;
    let ec: PatternNet<{ WorkflowPattern::ExclusiveChoice }> = PatternNet;

    // Each const-generic instantiation is a distinct type — they cannot be
    // exchanged.
    parallel_split_only(ps);
    exclusive_choice_only(ec);

    // All 17 pattern variants are constructible.
    let _: PatternNet<{ WorkflowPattern::Sequence }> = PatternNet;
    let _: PatternNet<{ WorkflowPattern::Synchronization }> = PatternNet;
    let _: PatternNet<{ WorkflowPattern::SimpleMerge }> = PatternNet;
    let _: PatternNet<{ WorkflowPattern::MultiChoice }> = PatternNet;
    let _: PatternNet<{ WorkflowPattern::StructuredSynchronizingMerge }> = PatternNet;
    let _: PatternNet<{ WorkflowPattern::MultiMerge }> = PatternNet;
    let _: PatternNet<{ WorkflowPattern::StructuredDiscriminator }> = PatternNet;
    let _: PatternNet<{ WorkflowPattern::ArbitraryCycles }> = PatternNet;
    let _: PatternNet<{ WorkflowPattern::ImplicitTermination }> = PatternNet;
    let _: PatternNet<{ WorkflowPattern::MultipleInstancesWithoutSync }> = PatternNet;
    let _: PatternNet<{ WorkflowPattern::MultipleInstancesWithDesignTimeKnowledge }> = PatternNet;
    let _: PatternNet<{ WorkflowPattern::DeferredChoice }> = PatternNet;
    let _: PatternNet<{ WorkflowPattern::InterleavedParallelRouting }> = PatternNet;
    let _: PatternNet<{ WorkflowPattern::CancelActivity }> = PatternNet;
    let _: PatternNet<{ WorkflowPattern::CancelCase }> = PatternNet;
}
