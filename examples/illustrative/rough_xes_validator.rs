//! A "rough" concrete XES interchange validator.
//!
//! This example demonstrates concrete enforcement of the `XesRefusal` laws,
//! specifically checking if all namespaced attributes in `XesEvent` are backed
//! by a declared extension in the `XesLog`.

use wasm4pm_compat::xes::{XesEvent, XesExtension, XesLog, XesRefusal, XesTrace};

/// A rough validator that performs a structural check on a XES log.
///
/// While `XesLog::validate` already performs these checks, this implementation
/// demonstrates how to manually inspect and enforce the XES interchange laws.
struct RoughXesValidator<'a> {
    log: &'a XesLog,
}

impl<'a> RoughXesValidator<'a> {
    fn new(log: &'a XesLog) -> Self {
        Self { log }
    }

    /// Validate the log and return a detailed report of any violations.
    fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // 1. Check Log Name (MissingLogName law)
        if self.log.name().is_empty() {
            errors.push(format!("Violation: {}", XesRefusal::MissingLogName));
        }

        // 2. Check Extensions (InvalidExtension law)
        let mut declared_prefixes = Vec::new();
        for ext in self.log.extensions() {
            if ext.prefix().is_empty() {
                errors.push(format!(
                    "Violation: {} (extension '{}' has empty prefix)",
                    XesRefusal::InvalidExtension,
                    ext.name()
                ));
            } else {
                declared_prefixes.push(ext.prefix());
            }
        }

        // 3. Check Traces (NoTraces, MissingTraceName, EmptyTrace laws)
        if self.log.traces().is_empty() {
            errors.push(format!("Violation: {}", XesRefusal::NoTraces));
        }

        for (t_idx, trace) in self.log.traces().iter().enumerate() {
            if trace.name().is_empty() {
                errors.push(format!(
                    "Violation: {} at trace index {}",
                    XesRefusal::MissingTraceName,
                    t_idx
                ));
            }

            if trace.is_empty() {
                errors.push(format!(
                    "Violation: {} for trace '{}'",
                    XesRefusal::EmptyTrace,
                    trace.name()
                ));
            }

            // 4. Check Events (MissingConceptName, UndeclaredExtensionPrefix laws)
            for (e_idx, event) in trace.events().iter().enumerate() {
                if event.concept_name().is_none() {
                    errors.push(format!(
                        "Violation: {} in trace '{}', event index {}",
                        XesRefusal::MissingConceptName,
                        trace.name(),
                        e_idx
                    ));
                }

                for (key, _) in event.attributes() {
                    if key.contains(':') {
                        let prefix = key.split(':').next().unwrap_or("");
                        if !prefix.is_empty() && !declared_prefixes.contains(&prefix) {
                            errors.push(format!(
                                "Violation: {} (attribute '{}' in trace '{}', event index {} uses undeclared prefix '{}')",
                                XesRefusal::UndeclaredExtensionPrefix,
                                key,
                                trace.name(),
                                e_idx,
                                prefix
                            ));
                        }
                    }
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

fn main() {
    println!("--- Rough XES Validator ---");

    // Case 1: A valid log
    println!("\nValidating Case 1: Standard Concept/Time/Lifecycle/Org log...");
    let valid_log = XesLog::new(
        "StandardLog",
        [
            XesExtension::new(
                "Concept",
                "concept",
                "http://www.xes-standard.org/concept.xesext",
            ),
            XesExtension::new("Time", "time", "http://www.xes-standard.org/time.xesext"),
        ],
        [XesTrace::new(
            "case-001",
            [XesEvent::new()
                .with("concept:name", "Register")
                .with("time:timestamp", "2026-05-30T10:00:00Z")],
        )],
    );

    let validator = RoughXesValidator::new(&valid_log);
    match validator.validate() {
        Ok(_) => println!("✅ Log is structurally valid."),
        Err(e) => {
            println!("❌ Log validation failed:");
            for err in e {
                println!("  - {}", err);
            }
        }
    }

    // Case 2: An invalid log with undeclared extensions and missing names
    println!("\nValidating Case 2: Log with law violations...");
    let invalid_log = XesLog::new(
        "", // MissingLogName
        [XesExtension::new("Concept", "concept", "uri")],
        [
            XesTrace::new(
                "case-777",
                [
                    XesEvent::new()
                        .with("concept:name", "Action")
                        .with("custom:extra", "data"), // UndeclaredExtensionPrefix
                    XesEvent::new(), // MissingConceptName
                ],
            ),
            XesTrace::new("", []), // MissingTraceName and EmptyTrace
        ],
    );

    let validator = RoughXesValidator::new(&invalid_log);
    match validator.validate() {
        Ok(_) => println!("✅ Log is structurally valid."),
        Err(e) => {
            println!("❌ Log validation failed (Expected):");
            for err in e {
                println!("  - {}", err);
            }
        }
    }
}
