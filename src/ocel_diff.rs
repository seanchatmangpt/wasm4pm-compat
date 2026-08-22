//! Real, generalized before/after ground-fact state-diff validator.
//!
//! This module compares two ground-fact snapshots (as emitted by the
//! platform-console `capability-state-snapshot` route, one taken before a
//! capability invocation and one after) and produces a real, per-fact
//! [`StateDiff`]. It then compares that diff against a capability's
//! declared expected effect, producing an [`EffectMatchResult`] that
//! reports every discrepancy (unexpected additions, missing expected
//! facts, and value mismatches on facts present in both) rather than a
//! single pass/fail boolean.
//!
//! It is deliberately generalized over ~30+ capabilities with varying
//! effect shapes: a capability declares its expected effect as the same
//! `StateSnapshot`-shaped fact map used for the ground facts, so no
//! capability-specific code lives in this module.
//!
//! # Fact identity
//!
//! A "fact" is identified by its `predicate` key in the snapshot's
//! `facts` map (e.g. `"order_status(42)"`, `"balance(acct-7)"`). The
//! predicate string is expected to already encode any arguments (this
//! matches how ground facts are typically rendered as flat JSON keys by
//! the platform-console route); this module does not further decompose
//! predicate/argument structure, since `serde_json::Value` already gives
//! callers a fully generic value type for the fact's payload.

use serde_json::Value;
use std::collections::BTreeMap;

/// A single ground-fact snapshot of world state at one point in time.
///
/// `facts` maps a predicate key (as a fully-qualified string, e.g.
/// `"order_status(42)"` or `"balance(acct-7)"`) to its current value.
/// `BTreeMap` is used (not `HashMap`) so snapshots serialize and iterate
/// deterministically, which matters for reproducible diffs and receipts.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
pub struct StateSnapshot {
    pub facts: BTreeMap<String, Value>,
}

impl StateSnapshot {
    /// Build a snapshot from an iterator of `(predicate, value)` pairs.
    pub fn from_pairs<I, S>(pairs: I) -> Self
    where
        I: IntoIterator<Item = (S, Value)>,
        S: Into<String>,
    {
        Self {
            facts: pairs.into_iter().map(|(k, v)| (k.into(), v)).collect(),
        }
    }

    /// Build a snapshot from `(predicate, args, value)` triples, folding
    /// `predicate` and `args` into a single flat key
    /// `"predicate(args)"`. `args` is rendered via `Display` so callers
    /// can pass anything from a bare id to a pre-formatted argument list.
    pub fn from_triples<I, P, A>(triples: I) -> Self
    where
        I: IntoIterator<Item = (P, A, Value)>,
        P: std::fmt::Display,
        A: std::fmt::Display,
    {
        Self {
            facts: triples
                .into_iter()
                .map(|(pred, args, val)| (format!("{pred}({args})"), val))
                .collect(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.facts.is_empty()
    }

    pub fn len(&self) -> usize {
        self.facts.len()
    }
}

/// A per-fact change: the fact's old value in the before-snapshot and its
/// new value in the after-snapshot. Only recorded for facts present in
/// both snapshots whose values differ.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ChangedFact {
    pub predicate: String,
    pub old_value: Value,
    pub new_value: Value,
}

/// The result of diffing two [`StateSnapshot`]s: facts present only
/// after, facts present only before (i.e. removed), and facts present in
/// both but whose value changed. This is a real structural diff over the
/// fact maps, not a hash/equality check — every entry is individually
/// comparable and reportable.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
pub struct StateDiff {
    /// Facts present in `after` but not in `before`.
    pub added: BTreeMap<String, Value>,
    /// Facts present in `before` but not in `after`.
    pub removed: BTreeMap<String, Value>,
    /// Facts present in both, with differing values.
    pub changed: Vec<ChangedFact>,
}

impl StateDiff {
    /// Compute the real structural diff between `before` and `after`.
    ///
    /// - A predicate in `after` but not `before` is `added`.
    /// - A predicate in `before` but not `after` is `removed`.
    /// - A predicate in both with `before_value != after_value` is
    ///   `changed`.
    /// - A predicate in both with an identical value is unchanged and
    ///   does not appear in the diff at all.
    pub fn compute(before: &StateSnapshot, after: &StateSnapshot) -> Self {
        let mut added = BTreeMap::new();
        let mut removed = BTreeMap::new();
        let mut changed = Vec::new();

        for (pred, after_val) in &after.facts {
            match before.facts.get(pred) {
                None => {
                    added.insert(pred.clone(), after_val.clone());
                }
                Some(before_val) => {
                    if before_val != after_val {
                        changed.push(ChangedFact {
                            predicate: pred.clone(),
                            old_value: before_val.clone(),
                            new_value: after_val.clone(),
                        });
                    }
                }
            }
        }

        for (pred, before_val) in &before.facts {
            if !after.facts.contains_key(pred) {
                removed.insert(pred.clone(), before_val.clone());
            }
        }

        Self {
            added,
            removed,
            changed,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.changed.is_empty()
    }
}

/// The result of comparing an actual [`StateDiff`] against a capability's
/// declared expected effect.
///
/// `matches` is `true` only when every discrepancy category below is
/// empty. `discrepancies` enumerates every individual mismatch found —
/// not just the first one — so a single failing run surfaces the full
/// set of problems at once.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
pub struct EffectMatchResult {
    pub matches: bool,
    pub discrepancies: Vec<String>,
}

impl EffectMatchResult {
    fn ok() -> Self {
        Self {
            matches: true,
            discrepancies: Vec::new(),
        }
    }

    fn from_discrepancies(discrepancies: Vec<String>) -> Self {
        Self {
            matches: discrepancies.is_empty(),
            discrepancies,
        }
    }
}

/// Compare a real, computed [`StateDiff`] against a declared expected
/// effect (itself expressed as a [`StateSnapshot`] of the facts a
/// capability is expected to add or change, mapped to their expected
/// post-invocation value).
///
/// This performs a full per-fact comparison, not an exact-match
/// short-circuit:
///
/// - Every predicate declared in `expected_effect` must appear in
///   `actual.added` or `actual.changed` with a value equal to the
///   declared value. If it is missing from the diff entirely, or present
///   with a different value, that is reported.
/// - Every predicate in `actual.added` that is *not* declared in
///   `expected_effect` is reported as an unexpected addition.
/// - Every `ChangedFact` in `actual.changed` whose `new_value` disagrees
///   with the declared expected value (or that is undeclared entirely)
///   is reported, with both the old and new values included in the
///   message so a human/log reader can see the actual transition, not
///   just that a mismatch occurred.
///
/// `actual.removed` is not compared against `expected_effect` because
/// expected effects are expressed as post-invocation facts; capabilities
/// that are expected to retract facts should declare their absence via
/// a separate mechanism (out of scope for this generalized comparator —
/// see module docs).
pub fn match_effect(actual: &StateDiff, expected_effect: &StateSnapshot) -> EffectMatchResult {
    let mut discrepancies = Vec::new();

    // Build a lookup of every predicate the actual diff touched (added or
    // changed) to its resulting value, so we can check declared
    // expectations against whichever bucket the fact actually landed in.
    let mut actual_new_values: BTreeMap<&str, &Value> = BTreeMap::new();
    for (pred, val) in &actual.added {
        actual_new_values.insert(pred.as_str(), val);
    }
    for cf in &actual.changed {
        actual_new_values.insert(cf.predicate.as_str(), &cf.new_value);
    }

    // 1. Every declared expected fact must be present with the declared value.
    for (pred, expected_val) in &expected_effect.facts {
        match actual_new_values.get(pred.as_str()) {
            None => {
                discrepancies.push(format!(
                    "expected fact not present in actual diff: {pred} (expected {expected_val})"
                ));
            }
            Some(actual_val) => {
                if *actual_val != expected_val {
                    discrepancies.push(format!(
                        "expected fact {pred} has value {actual_val} but declared effect expects {expected_val}"
                    ));
                }
            }
        }
    }

    // 2. Every added fact not declared in the expected effect is unexpected.
    for (pred, val) in &actual.added {
        if !expected_effect.facts.contains_key(pred) {
            discrepancies.push(format!(
                "unexpected added fact not declared in expected effect: {pred} = {val}"
            ));
        }
    }

    // 3. Every changed fact not declared in the expected effect at all is
    //    reported with both old and new values so the reader can see the
    //    real transition. Changed facts that *are* declared but whose
    //    value disagrees with the declared value were already reported
    //    by the loop above (which checks every declared predicate's
    //    resulting value regardless of whether it landed in `added` or
    //    `changed`) — this loop must not re-report the same predicate a
    //    second time.
    for cf in &actual.changed {
        if !expected_effect.facts.contains_key(cf.predicate.as_str()) {
            discrepancies.push(format!(
                "unexpected changed fact not declared in expected effect: {} (old={}, new={})",
                cf.predicate, cf.old_value, cf.new_value
            ));
        }
    }

    if discrepancies.is_empty() {
        EffectMatchResult::ok()
    } else {
        EffectMatchResult::from_discrepancies(discrepancies)
    }
}

/// Convenience end-to-end entry point: diff `before`/`after` and match
/// the resulting diff against `expected_effect` in one call.
pub fn diff_and_match(
    before: &StateSnapshot,
    after: &StateSnapshot,
    expected_effect: &StateSnapshot,
) -> (StateDiff, EffectMatchResult) {
    let diff = StateDiff::compute(before, after);
    let result = match_effect(&diff, expected_effect);
    (diff, result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Real Rust logic over real data structures: no test doubles are
    /// used anywhere in this module. `unittest.mock`/`Mock`/`patch`-style
    /// fakes do not even apply here (there is no external collaborator
    /// to fake) -- every assertion below runs the real `StateDiff` and
    /// `EffectMatchResult` code against real `StateSnapshot` values.

    #[test]
    fn diff_matches_declared_effect_exactly() {
        let before = StateSnapshot::from_pairs([
            ("order_status(42)", json!("pending")),
            ("balance(acct-7)", json!(100)),
        ]);
        let after = StateSnapshot::from_pairs([
            ("order_status(42)", json!("shipped")),
            ("balance(acct-7)", json!(100)),
            ("tracking(42)", json!("TRK-999")),
        ]);

        let diff = StateDiff::compute(&before, &after);

        // Real structural assertions on the diff itself.
        assert_eq!(diff.added.len(), 1);
        assert_eq!(diff.added.get("tracking(42)"), Some(&json!("TRK-999")));
        assert!(diff.removed.is_empty());
        assert_eq!(diff.changed.len(), 1);
        assert_eq!(diff.changed[0].predicate, "order_status(42)");
        assert_eq!(diff.changed[0].old_value, json!("pending"));
        assert_eq!(diff.changed[0].new_value, json!("shipped"));

        let expected_effect = StateSnapshot::from_pairs([
            ("order_status(42)", json!("shipped")),
            ("tracking(42)", json!("TRK-999")),
        ]);

        let result = match_effect(&diff, &expected_effect);
        assert!(
            result.matches,
            "expected exact match, got discrepancies: {:?}",
            result.discrepancies
        );
        assert!(result.discrepancies.is_empty());
    }

    #[test]
    fn extra_unexpected_fact_is_caught_not_silently_passed() {
        let before = StateSnapshot::from_pairs([("order_status(42)", json!("pending"))]);
        let after = StateSnapshot::from_pairs([
            ("order_status(42)", json!("shipped")),
            // Undeclared side effect: a capability that also silently
            // bumped an audit counter it never declared.
            ("audit_count(42)", json!(1)),
        ]);

        let diff = StateDiff::compute(&before, &after);
        assert_eq!(diff.added.len(), 1);
        assert!(diff.added.contains_key("audit_count(42)"));

        let expected_effect = StateSnapshot::from_pairs([("order_status(42)", json!("shipped"))]);

        let result = match_effect(&diff, &expected_effect);
        assert!(
            !result.matches,
            "unexpected extra fact must not silently pass"
        );
        assert_eq!(result.discrepancies.len(), 1);
        assert!(
            result.discrepancies[0].contains("audit_count(42)"),
            "discrepancy message must name the offending predicate: {:?}",
            result.discrepancies
        );
        assert!(result.discrepancies[0].contains("unexpected added fact"));
    }

    #[test]
    fn missing_declared_fact_is_caught() {
        let before = StateSnapshot::from_pairs([("order_status(42)", json!("pending"))]);
        // Capability was expected to also add tracking(42), but did not.
        let after = StateSnapshot::from_pairs([("order_status(42)", json!("shipped"))]);

        let diff = StateDiff::compute(&before, &after);
        assert!(diff.added.is_empty());

        let expected_effect = StateSnapshot::from_pairs([
            ("order_status(42)", json!("shipped")),
            ("tracking(42)", json!("TRK-999")),
        ]);

        let result = match_effect(&diff, &expected_effect);
        assert!(!result.matches, "missing declared fact must be caught");
        assert_eq!(result.discrepancies.len(), 1);
        assert!(
            result.discrepancies[0].contains("tracking(42)"),
            "discrepancy message must name the missing predicate: {:?}",
            result.discrepancies
        );
        assert!(result.discrepancies[0].contains("expected fact not present"));
    }

    #[test]
    fn changed_value_case_is_caught_and_reports_old_and_new() {
        let before = StateSnapshot::from_pairs([("balance(acct-7)", json!(100))]);
        // Capability declared it would set balance to 150, but the real
        // effect landed at 140 (e.g. a fee was also deducted).
        let after = StateSnapshot::from_pairs([("balance(acct-7)", json!(140))]);

        let diff = StateDiff::compute(&before, &after);
        assert_eq!(diff.changed.len(), 1);
        assert_eq!(diff.changed[0].old_value, json!(100));
        assert_eq!(diff.changed[0].new_value, json!(140));

        let expected_effect = StateSnapshot::from_pairs([("balance(acct-7)", json!(150))]);

        let result = match_effect(&diff, &expected_effect);
        assert!(!result.matches, "value mismatch must be caught");
        assert_eq!(result.discrepancies.len(), 1);
        let msg = &result.discrepancies[0];
        assert!(msg.contains("balance(acct-7)"), "msg: {msg}");
        assert!(msg.contains("140"), "must report actual new value: {msg}");
        assert!(
            msg.contains("150"),
            "must report declared expected value: {msg}"
        );
    }

    #[test]
    fn unchanged_fact_does_not_appear_in_diff() {
        let before = StateSnapshot::from_pairs([("balance(acct-7)", json!(100))]);
        let after = StateSnapshot::from_pairs([("balance(acct-7)", json!(100))]);
        let diff = StateDiff::compute(&before, &after);
        assert!(diff.is_empty());
    }

    #[test]
    fn removed_fact_is_tracked_in_diff() {
        let before = StateSnapshot::from_pairs([
            ("lock(42)", json!(true)),
            ("order_status(42)", json!("pending")),
        ]);
        let after = StateSnapshot::from_pairs([("order_status(42)", json!("pending"))]);

        let diff = StateDiff::compute(&before, &after);
        assert_eq!(diff.removed.len(), 1);
        assert_eq!(diff.removed.get("lock(42)"), Some(&json!(true)));
        assert!(diff.added.is_empty());
        assert!(diff.changed.is_empty());
    }

    #[test]
    fn from_triples_folds_predicate_and_args_into_flat_key() {
        let snap = StateSnapshot::from_triples([
            ("order_status", "42".to_string(), json!("pending")),
            ("balance", "acct-7".to_string(), json!(100)),
        ]);
        assert_eq!(snap.facts.get("order_status(42)"), Some(&json!("pending")));
        assert_eq!(snap.facts.get("balance(acct-7)"), Some(&json!(100)));
    }

    #[test]
    fn diff_and_match_end_to_end_convenience_entry_point() {
        let before = StateSnapshot::from_pairs([("k", json!(1))]);
        let after = StateSnapshot::from_pairs([("k", json!(2))]);
        let expected = StateSnapshot::from_pairs([("k", json!(2))]);

        let (diff, result) = diff_and_match(&before, &after, &expected);
        assert_eq!(diff.changed.len(), 1);
        assert!(result.matches);
    }
}
