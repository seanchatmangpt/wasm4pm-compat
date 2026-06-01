// COMPILE-PASS: Temporal context typed — OnlineEvidence and OfflineEvidence are distinct types
// and each can be constructed and used in context-specific positions.
//
// Law: TemporalContextLaw — ContextualEvidence context tags are enforced at the type level.
use wasm4pm_compat::streaming::{
    ContextualEvidence, OfflineAnalysisContext, OfflineEvidence, OnlineEvidence,
    OnlineMonitoringContext,
};

fn process_online(_ev: ContextualEvidence<u64, OnlineMonitoringContext>) {}
fn process_offline(_ev: ContextualEvidence<u64, OfflineAnalysisContext>) {}

fn main() {
    let online: OnlineEvidence<u64> = ContextualEvidence::online(1u64);
    let offline: OfflineEvidence<u64> = ContextualEvidence::offline(2u64);

    // Each context type is accepted by its matching function and rejected by the other
    process_online(online);
    process_offline(offline);
}
