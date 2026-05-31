// COMPILE-PASS: xes-lifecycle-transition-full-alphabet — proves XesLifecycleTransition
// names the full IEEE 1849-2016 standard alphabet (14 variants), that as_str()
// is consistent with parse(), and that is_terminal() identifies the terminal states.
use wasm4pm_compat::xes::XesLifecycleTransition;

fn main() {
    // Full alphabet round-trip: parse(as_str(v)) == Some(v).
    let all = [
        XesLifecycleTransition::Schedule,
        XesLifecycleTransition::Assign,
        XesLifecycleTransition::Start,
        XesLifecycleTransition::Suspend,
        XesLifecycleTransition::Resume,
        XesLifecycleTransition::InProgress,
        XesLifecycleTransition::Abort,
        XesLifecycleTransition::Withdraw,
        XesLifecycleTransition::Complete,
        XesLifecycleTransition::Unknown,
        XesLifecycleTransition::AutoSkip,
        XesLifecycleTransition::ManualSkip,
        XesLifecycleTransition::Reassign,
        XesLifecycleTransition::Plan,
    ];
    assert_eq!(all.len(), 14);
    for v in all {
        let s = v.as_str();
        assert_eq!(XesLifecycleTransition::parse(s), Some(v));
    }

    // Terminal states.
    assert!(XesLifecycleTransition::Complete.is_terminal());
    assert!(XesLifecycleTransition::Abort.is_terminal());
    assert!(XesLifecycleTransition::Withdraw.is_terminal());
    assert!(XesLifecycleTransition::ManualSkip.is_terminal());
    assert!(XesLifecycleTransition::AutoSkip.is_terminal());

    // Non-terminal states.
    assert!(!XesLifecycleTransition::Start.is_terminal());
    assert!(!XesLifecycleTransition::Schedule.is_terminal());

    // Non-standard returns None.
    assert_eq!(XesLifecycleTransition::parse("nonstandard"), None);

    // Display delegates to as_str().
    assert_eq!(format!("{}", XesLifecycleTransition::Complete), "complete");
}
