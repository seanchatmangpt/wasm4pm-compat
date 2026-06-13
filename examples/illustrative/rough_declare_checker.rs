//! Example: A "rough" Declare constraint checker
//!
//! Demonstrates a simplified implementation of a Declare conformance checker.
//! This is "rough" because it uses basic iteration instead of a full LTL
//! or finite-state-machine engine.
//!
//! Goal:
//! 1. Create a Declare model with basic constraints.
//! 2. Implement a checker that marks constraints as violated/satisfied for a trace.
//! 3. Use the `DeclareConstraint` types from `src/declare.rs`.
//!
//! Run: cargo run --example rough_declare_checker

use wasm4pm_compat::declare::{Activity, DeclareConstraint, DeclareScope, DeclareTemplate};
use wasm4pm_compat::eventlog::{Event, Trace};

/// A simplified Declare checker for finite traces.
struct RoughDeclareChecker {
    constraints: Vec<DeclareConstraint>,
}

impl RoughDeclareChecker {
    pub fn new(constraints: Vec<DeclareConstraint>) -> Self {
        Self { constraints }
    }

    /// Checks all constraints against a single trace.
    /// Returns a list of (constraint, is_satisfied).
    pub fn check(&self, trace: &Trace) -> Vec<(&DeclareConstraint, bool)> {
        let events = trace.events();
        self.constraints
            .iter()
            .map(|c| {
                let satisfied = match c.template {
                    DeclareTemplate::Existence => self.check_existence(events, &c.activation),
                    DeclareTemplate::Absence => self.check_absence(events, &c.activation),
                    DeclareTemplate::Response => {
                        let target = c.target.as_ref().expect("Response requires a target");
                        self.check_response(events, &c.activation, target)
                    }
                    DeclareTemplate::Precedence => {
                        let target = c.target.as_ref().expect("Precedence requires a target");
                        self.check_precedence(events, &c.activation, target)
                    }
                    _ => {
                        // For this "rough" example, we only implement a few templates.
                        false
                    }
                };
                (c, satisfied)
            })
            .collect()
    }

    fn check_existence(&self, events: &[Event], activity: &Activity) -> bool {
        events.iter().any(|e| e.activity() == activity.name())
    }

    fn check_absence(&self, events: &[Event], activity: &Activity) -> bool {
        !events.iter().any(|e| e.activity() == activity.name())
    }

    fn check_response(&self, events: &[Event], activation: &Activity, target: &Activity) -> bool {
        for (i, event) in events.iter().enumerate() {
            if event.activity() == activation.name() {
                // If activation occurs, target must occur after it.
                let has_target_after = events[i + 1..]
                    .iter()
                    .any(|e| e.activity() == target.name());
                if !has_target_after {
                    return false;
                }
            }
        }
        true
    }

    fn check_precedence(&self, events: &[Event], activation: &Activity, target: &Activity) -> bool {
        for (i, event) in events.iter().enumerate() {
            if event.activity() == target.name() {
                // If target occurs, activation must have occurred before it.
                let has_activation_before = events[..i]
                    .iter()
                    .any(|e| e.activity() == activation.name());
                if !has_activation_before {
                    return false;
                }
            }
        }
        true
    }
}

fn main() {
    println!("=== Rough Declare Constraint Checker ===\n");

    // 1. Define Activities
    let login = Activity::new("login");
    let search = Activity::new("search");
    let pay = Activity::new("pay");
    let logout = Activity::new("logout");
    let error = Activity::new("error");

    // 2. Define Constraints (The Model)
    let constraints = vec![
        // Existence(login): login must occur.
        DeclareConstraint::unary(
            DeclareTemplate::Existence,
            login.clone(),
            DeclareScope::SingleObjectScope("session".into()),
        ),
        // Absence(error): error must not occur.
        DeclareConstraint::unary(
            DeclareTemplate::Absence,
            error.clone(),
            DeclareScope::SingleObjectScope("session".into()),
        ),
        // Response(pay, logout): every payment must be followed by a logout.
        DeclareConstraint::binary(
            DeclareTemplate::Response,
            pay.clone(),
            logout.clone(),
            DeclareScope::SingleObjectScope("session".into()),
        ),
        // Precedence(login, search): search must be preceded by login.
        DeclareConstraint::binary(
            DeclareTemplate::Precedence,
            login.clone(),
            search.clone(),
            DeclareScope::SingleObjectScope("session".into()),
        ),
    ];

    let checker = RoughDeclareChecker::new(constraints);

    // 3. Define Sample Traces
    let traces = vec![
        Trace::new(
            "trace-happy",
            vec![
                Event::new("login"),
                Event::new("search"),
                Event::new("pay"),
                Event::new("logout"),
            ],
        ),
        Trace::new(
            "trace-violated-response",
            vec![
                Event::new("login"),
                Event::new("pay"), // Missing logout after pay
            ],
        ),
        Trace::new(
            "trace-violated-precedence",
            vec![
                Event::new("search"), // Missing login before search
                Event::new("pay"),
                Event::new("logout"),
            ],
        ),
        Trace::new(
            "trace-violated-absence",
            vec![
                Event::new("login"),
                Event::new("error"), // Forbidden activity
                Event::new("logout"),
            ],
        ),
    ];

    // 4. Run Checker
    for trace in traces {
        println!("Checking Trace: {}", trace.case_id());
        let results = checker.check(&trace);
        for (constraint, satisfied) in results {
            let status = if satisfied {
                "✅ SATISFIED"
            } else {
                "❌ VIOLATED"
            };
            let target_str = constraint
                .target
                .as_ref()
                .map(|t| format!(", {}", t.name()))
                .unwrap_or_default();
            println!(
                "  {} : {:?}({}{})",
                status,
                constraint.template,
                constraint.activation.name(),
                target_str
            );
        }
        println!();
    }
}
