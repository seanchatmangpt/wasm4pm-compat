//! Example: Declare and OC-Declare constraint model construction
//!
//! Demonstrates how to build a Declare constraint model for a simple
//! order-fulfillment process: binary constraints (Response, Precedence,
//! Succession), unary existence constraints, and the OC-Declare extension
//! that scopes constraints to named object types.
//!
//! This example is **structure-only**. It builds and inspects constraint shapes;
//! it never evaluates them against an event log. Graduate to `wasm4pm` for
//! conformance checking, mining, and LTL replay.
//!
//! Run: cargo run --example declare_constraint_model

#![allow(dead_code)]

use wasm4pm_compat::declare::{
    Activity, DeclareConstraint, DeclareRefusal, DeclareScope, DeclareTemplate,
    OcDeclareConstraint, OcDeclareRefusal,
};

fn main() {
    println!("=== Declare Constraint Model: Order Fulfillment Process ===\n");

    // ── Activities ─────────────────────────────────────────────────────────
    // Every constraint references named activities. Activity is a transparent
    // newtype over String — a structural label, not an event instance.
    let submit = Activity::new("submit_order");
    let approve = Activity::new("approve_order");
    let pick = Activity::new("pick_items");
    let ship = Activity::new("ship_order");
    let deliver = Activity::new("deliver_order");
    let cancel = Activity::new("cancel_order");

    // ── Response: A response B ─────────────────────────────────────────────
    // Response(submit, approve): every submission must eventually be followed
    // by an approval. This is a positive binary constraint.
    let response = DeclareConstraint::binary(
        DeclareTemplate::Response,
        submit.clone(),
        approve.clone(),
        DeclareScope::SingleObjectScope("order".into()),
    );
    println!("Response constraint:");
    println!(
        "  template  = {:?}  (arity {})",
        response.template,
        response.template.arity()
    );
    println!("  activation = {:?}", response.activation);
    println!("  target     = {:?}", response.target);
    println!("  is_negative? {}", response.template.is_negative());
    println!("  is_chain?    {}", response.template.is_chain());
    println!();

    // ── Precedence: C precedes D ──────────────────────────────────────────
    // Precedence(pick, ship): every shipment must be preceded by a pick.
    // Note the role reversal: activation is the antecedent (pick), target is
    // the consequent (ship).
    let precedence = DeclareConstraint::binary(
        DeclareTemplate::Precedence,
        pick.clone(),
        ship.clone(),
        DeclareScope::SingleObjectScope("order".into()),
    );
    println!("Precedence constraint:");
    println!(
        "  template  = {:?}  (arity {})",
        precedence.template,
        precedence.template.arity()
    );
    println!("  activation = {:?}", precedence.activation);
    println!("  target     = {:?}", precedence.target);
    println!();

    // ── Succession ────────────────────────────────────────────────────────
    // Succession(ship, deliver): both Response and Precedence hold between
    // ship and deliver. Every shipment is eventually followed by a delivery,
    // and every delivery was preceded by a shipment.
    let succession = DeclareConstraint::binary(
        DeclareTemplate::Succession,
        ship.clone(),
        deliver.clone(),
        DeclareScope::SingleObjectScope("order".into()),
    );
    println!("Succession constraint (= Response ∧ Precedence):");
    println!(
        "  template  = {:?}  (arity {})",
        succession.template,
        succession.template.arity()
    );
    println!("  activation = {:?}", succession.activation);
    println!("  target     = {:?}", succession.target);
    println!();

    // ── Unary constraint: Absence ─────────────────────────────────────────
    // Absence(cancel): cancel_order must not occur (happy-path model).
    // Unary templates take only an activation; target is always None.
    let no_cancel = DeclareConstraint::unary(
        DeclareTemplate::Absence,
        cancel.clone(),
        DeclareScope::SingleObjectScope("order".into()),
    );
    println!("Absence constraint (unary, negative):");
    println!(
        "  template  = {:?}  (arity {})",
        no_cancel.template,
        no_cancel.template.arity()
    );
    println!("  activation = {:?}", no_cancel.activation);
    println!("  target     = {:?}  (always None for unary)", no_cancel.target);
    println!("  is_negative? {}", no_cancel.template.is_negative());
    println!();

    // ── OcDeclareConstraint: object-scoped variant ─────────────────────────
    // OC-Declare scopes a constraint to one or more named object types.
    // Here the Response(submit, approve) constraint is bound to both "order"
    // and "customer" object types, applied independently (not synchronized).
    let inner_response = DeclareConstraint::binary(
        DeclareTemplate::Response,
        submit.clone(),
        approve.clone(),
        DeclareScope::MultiObjectScope(vec!["order".into(), "customer".into()]),
    );
    let oc_response = OcDeclareConstraint::new(
        inner_response,
        vec!["order".into(), "customer".into()],
    );
    println!("OcDeclareConstraint (non-synchronized, multi-object):");
    println!(
        "  template      = {:?}",
        oc_response.constraint.template
    );
    println!("  object_types  = {:?}", oc_response.object_types);
    println!("  synchronized  = {}", oc_response.is_synchronized());
    println!("  validate()    = {:?}", oc_response.validate());
    println!();

    // ── OcDeclareConstraint: synchronized variant ─────────────────────────
    // Synchronized OC-Declare requires a joint lifecycle across all named
    // object types. Succession(ship, deliver) synchronized over "order" and
    // "delivery" means both objects must participate together.
    let inner_succession = DeclareConstraint::binary(
        DeclareTemplate::Succession,
        ship.clone(),
        deliver.clone(),
        DeclareScope::SynchronizedObjectScope(vec!["order".into(), "delivery".into()]),
    );
    let oc_synchronized = OcDeclareConstraint::synchronized(
        inner_succession,
        vec!["order".into(), "delivery".into()],
    );
    println!("OcDeclareConstraint (synchronized, multi-object):");
    println!(
        "  template      = {:?}",
        oc_synchronized.constraint.template
    );
    println!("  object_types  = {:?}", oc_synchronized.object_types);
    println!("  synchronized  = {}", oc_synchronized.is_synchronized());
    println!("  validate()    = {:?}", oc_synchronized.validate());
    println!();

    // ── Structural validation: refusal surfaces ───────────────────────────
    // OcDeclareConstraint::validate() refuses an empty object_types list
    // with a named law — never a bare "InvalidInput".
    let bad_inner = DeclareConstraint::unary(
        DeclareTemplate::Existence,
        submit.clone(),
        DeclareScope::SingleObjectScope("order".into()),
    );
    let empty_oc = OcDeclareConstraint::new(bad_inner, vec![]);
    let refusal = empty_oc.validate().unwrap_err();
    println!("Refusal (EmptyObjectTypeList):");
    println!("  {:?}", refusal);
    println!("  Display: {}", refusal);
    assert_eq!(refusal, OcDeclareRefusal::EmptyObjectTypeList);
    println!();

    // ── Composing constraints into a model (Vec) ──────────────────────────
    // DeclareConstraint has no special container type — a Vec is the
    // structural composition form until a named DeclareModel type graduates
    // to wasm4pm.
    let model: Vec<DeclareConstraint> = vec![
        response.clone(),
        precedence.clone(),
        succession.clone(),
        no_cancel.clone(),
    ];

    println!("Composed model ({} constraints):", model.len());
    for (i, c) in model.iter().enumerate() {
        let target_label = c
            .target
            .as_ref()
            .map(|t| t.0.as_str())
            .unwrap_or("—");
        println!(
            "  [{i}] {:?}({}, {target_label})  negative={} chain={}",
            c.template,
            c.activation.0,
            c.template.is_negative(),
            c.template.is_chain(),
        );
    }
    println!();

    // ── DeclareRefusal: named laws ────────────────────────────────────────
    // The DeclareRefusal enum provides named laws for constraint-shape
    // failures. Each variant is specific — no bare catch-alls.
    let sample_refusals = [
        DeclareRefusal::MissingActivation,
        DeclareRefusal::MissingTarget,
        DeclareRefusal::InvalidTemplateArity,
        DeclareRefusal::EmptyObjectScope,
        DeclareRefusal::SynchronizationViolation,
    ];
    println!("DeclareRefusal named laws:");
    for r in &sample_refusals {
        println!("  {}", r);
    }

    println!("\nDone. All constraint shapes are structure-only; graduate to wasm4pm for evaluation.");
}
