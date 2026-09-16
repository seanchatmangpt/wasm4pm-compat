//! Behavioral receipts for the `AbstractionLevel` marker law.
//!
//! Paper #52 — Baier, De Weerdt & Senderovich (2022), *Event Abstraction in
//! Process Mining: Literature Review and Taxonomy*: the low-level vs
//! high-level activity distinction is a named structural property of the log
//! type, not a free string rename.

use wasm4pm_compat::eventlog::{
    AbstractedEventLog, AbstractionLevel, Event, EventLog, HighLevelLog, LowLevelLog, Trace,
};

/// The compile-time level constants are distinct and classify correctly.
#[test]
fn level_constants_classify_correctly() {
    assert_eq!(LowLevelLog::level(), AbstractionLevel::Low);
    assert_eq!(HighLevelLog::level(), AbstractionLevel::High);
    assert!(LowLevelLog::level().is_low());
    assert!(HighLevelLog::level().is_high());
    assert!(!LowLevelLog::level().is_high());
    assert!(!HighLevelLog::level().is_low());
}

/// `as_str` and `Display` agree and are stable receipt vocabulary.
#[test]
fn as_str_and_display_agree() {
    for level in [AbstractionLevel::Low, AbstractionLevel::High] {
        assert_eq!(level.to_string(), level.as_str());
    }
    assert_eq!(AbstractionLevel::Low.to_string(), "Low");
    assert_eq!(AbstractionLevel::High.to_string(), "High");
}

/// Tagging is zero-cost: the wrapper has exactly the size of the inner log.
#[test]
fn wrapper_is_repr_transparent() {
    assert_eq!(
        core::mem::size_of::<AbstractedEventLog<{ AbstractionLevel::Low }>>(),
        core::mem::size_of::<EventLog>()
    );
    assert_eq!(
        core::mem::size_of::<AbstractedEventLog<{ AbstractionLevel::High }>>(),
        core::mem::size_of::<EventLog>()
    );
}

/// The level tag neither mutates nor loses the underlying log.
#[test]
fn level_tag_round_trips_the_log() {
    let log = EventLog::from_traces([Trace::new(
        "case-1",
        [
            Event::new("create_requisition").at_ns(1_000),
            Event::new("approve_requisition").at_ns(2_000),
        ],
    )]);
    let tagged = LowLevelLog::new(log.clone());
    assert_eq!(tagged.inner(), &log);
    assert_eq!(tagged.inner().trace_count(), 1);
    assert_eq!(tagged.inner().event_count(), 2);
    assert_eq!(tagged.into_inner(), log);
}

/// The two level aliases are usable as distinct types at the same call site:
/// compiling this function is the positive proof that the const-generic
/// parameters resolve to different types, not one erased label.
#[test]
fn level_aliases_flow_as_distinct_types() {
    fn takes_low(_: &LowLevelLog) -> AbstractionLevel {
        LowLevelLog::level()
    }
    fn takes_high(_: &HighLevelLog) -> AbstractionLevel {
        HighLevelLog::level()
    }

    let low = LowLevelLog::new(EventLog::default());
    let high = HighLevelLog::new(EventLog::default());
    assert_eq!(takes_low(&low), AbstractionLevel::Low);
    assert_eq!(takes_high(&high), AbstractionLevel::High);
}

/// The marker composes with the existing validation law unchanged.
#[test]
fn tagged_log_validates_through_inner() {
    let valid = LowLevelLog::new(EventLog::from_traces([Trace::new(
        "case-1",
        [Event::new("a").at_ns(1)],
    )]));
    assert!(valid.inner().validate().is_ok());

    let empty_trace = HighLevelLog::new(EventLog::from_traces([Trace::new("case-1", [])]));
    assert_eq!(
        empty_trace.inner().validate(),
        Err(wasm4pm_compat::eventlog::EventLogRefusal::EmptyTrace)
    );
}
