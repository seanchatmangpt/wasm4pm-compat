#![feature(generic_const_exprs, adt_const_params, unsized_const_params)]
#![allow(incomplete_features)]
// Law: CrossLogCorrelationLaw — CorrelationKey<SCHEMA>, CorrelationWitness<SCHEMA>, and
// CorrelatedLog<A, B, SCHEMA> are distinct types for distinct SCHEMA strings.

// COMPILE-PASS: Cross-log correlation shapes compile and schema const-generic
// parameters distinguish different correlation strategies at the type level.
//
// Law: CrossLogCorrelationLaw — CorrelationKey<"by-case"> and
// CorrelationKey<"by-object"> are different types; CorrelatedLog<A,B,"by-case">
// and CorrelatedLog<A,B,"by-object"> are different types; CorrelationSchema
// display is stable.

use wasm4pm_compat::correlation::{
    CorrelatedLog,
    CorrelationKey,
    CorrelationSchema,
    CorrelationWitness,
};

struct XesLog;
struct OcelLog;

fn check_correlation_key() {
    let key_case: CorrelationKey<"by-case"> = CorrelationKey::new();
    assert_eq!(key_case.schema(), "by-case");

    let key_obj: CorrelationKey<"by-object"> = CorrelationKey::new();
    assert_eq!(key_obj.schema(), "by-object");

    let key_ts: CorrelationKey<"by-timestamp"> = CorrelationKey::default();
    assert_eq!(key_ts.schema(), "by-timestamp");
}

fn check_correlation_witness() {
    // CorrelationWitness is zero-sized.
    assert_eq!(core::mem::size_of::<CorrelationWitness<"by-case">>(), 0);
    assert_eq!(core::mem::size_of::<CorrelationWitness<"by-object">>(), 0);
}

fn check_correlated_log() {
    let merged: CorrelatedLog<XesLog, OcelLog, "by-case"> = CorrelatedLog::new();
    assert_eq!(merged.schema(), "by-case");

    let merged_obj: CorrelatedLog<XesLog, OcelLog, "by-object"> = CorrelatedLog::default();
    assert_eq!(merged_obj.schema(), "by-object");

    let merged_attr: CorrelatedLog<XesLog, OcelLog, "by-attribute"> = CorrelatedLog::new();
    assert_eq!(merged_attr.schema(), "by-attribute");
}

fn check_correlation_schema_display() {
    assert_eq!(CorrelationSchema::ByCase.to_string(), "by-case");
    assert_eq!(CorrelationSchema::ByObject.to_string(), "by-object");
    assert_eq!(CorrelationSchema::ByTimestamp.to_string(), "by-timestamp");
    assert_eq!(CorrelationSchema::ByAttribute.to_string(), "by-attribute");
}

fn demands_by_case_only(_: CorrelationKey<"by-case">) {}

fn check_schema_specificity() {
    // Only a CorrelationKey<"by-case"> satisfies the by-case demand.
    let key: CorrelationKey<"by-case"> = CorrelationKey::new();
    demands_by_case_only(key);
    // (Passing CorrelationKey<"by-object"> would be a compile-fail — tested separately.)
}

fn main() {
    check_correlation_key();
    check_correlation_witness();
    check_correlated_log();
    check_correlation_schema_display();
    check_schema_specificity();
}
