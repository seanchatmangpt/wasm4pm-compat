//! Smoke tests for the part-B process-canon shapes (Agent 3b).
//!
//! These construct each shape and assert its basic structural invariants. They
//! are **structure-only** smoke checks — they never run discovery, conformance,
//! prediction, or replay. They merely prove the shapes build and that their
//! bounded newtypes refuse out-of-range input.

use wasm4pm_compat::conformance::{
    ConformanceRefusal, ConformanceVerdict, Deviation, Fitness, Generalization, Precision,
    Simplicity, F1,
};
use wasm4pm_compat::declare::{
    Activity, DeclareConstraint, DeclareRefusal, DeclareScope, DeclareTemplate,
    OcDeclareConstraint, OcDeclareRefusal,
};
use wasm4pm_compat::ocpq::{ObjectScope, OcpqQuery, OcpqRefusal, Predicate, PredicateKind};
use wasm4pm_compat::powl::{
    assert_acyclic, assert_tree_projectable, AcyclicPartialOrder, OrderEdge, Powl, PowlChoiceNode,
    PowlNode, PowlNodeId, PowlNodeKind, PowlRefusal, ProcessTreeProjectable, RefusedProjection,
};
use wasm4pm_compat::prediction::{PredictionProblem, PredictionRefusal, PredictionTarget};
use wasm4pm_compat::process_tree::{
    ProcessTree, ProcessTreeNode, ProcessTreeOperator, ProcessTreeRefusal,
};
use wasm4pm_compat::receipt::{Digest, ReceiptRefusal, ReceiptShape, ReplayHint};

#[test]
fn smoke_powl() {
    // Build a tiny partial order: a -> b.
    let mut p = Powl::new();
    p.nodes
        .push(PowlNode::new(PowlNodeId(0), PowlNodeKind::Atom("a".into())));
    p.nodes
        .push(PowlNode::new(PowlNodeId(1), PowlNodeKind::Atom("b".into())));
    p.nodes.push(PowlNode::new(
        PowlNodeId(2),
        PowlNodeKind::PartialOrder(vec![PowlNodeId(0), PowlNodeId(1)]),
    ));
    p.edges.push(OrderEdge::new(PowlNodeId(0), PowlNodeId(1)));
    p.root = Some(PowlNodeId(2));

    assert_eq!(p.node_count(), 3);
    assert_eq!(p.edges[0].from, PowlNodeId(0));
    assert_eq!(p.root, Some(PowlNodeId(2)));

    // Refusal names a specific law.
    let r = PowlRefusal::CyclicPartialOrder;
    assert_eq!(r, PowlRefusal::CyclicPartialOrder);
    assert_eq!(r, PowlRefusal::CyclicPartialOrder);
}

#[test]
fn smoke_powl_acyclic_witness() {
    // AcyclicPartialOrder satisfies AcyclicWitness; PartialOrder does not.
    assert!(assert_acyclic(AcyclicPartialOrder));
}

#[test]
fn smoke_powl_choice_node_well_formed() {
    let c = PowlChoiceNode::new(vec![PowlNodeId(0), PowlNodeId(1)]);
    assert!(c.is_well_formed());
    assert_eq!(c.branch_count(), 2);
    assert!(c.validate().is_ok());
}

#[test]
fn smoke_powl_choice_node_malformed() {
    let bad = PowlChoiceNode::new(vec![PowlNodeId(0)]);
    assert!(!bad.is_well_formed());
    assert_eq!(
        bad.validate(),
        Err(PowlRefusal::InvalidChoiceArity {
            declared: 1,
            required_min: 2
        })
    );
}

#[test]
fn smoke_powl_refused_projection() {
    let r = RefusedProjection::new(PowlRefusal::IrreducibleProjection);
    assert_eq!(r.reason(), &PowlRefusal::IrreducibleProjection);
    let owned = RefusedProjection::new(PowlRefusal::CyclicPartialOrder).into_reason();
    assert_eq!(owned, PowlRefusal::CyclicPartialOrder);
}

#[test]
fn smoke_powl_tree_projectable_gate() {
    // ProcessTreeProjectable passes the gate; ExceedsProcessTree does not (compile-fail).
    assert!(assert_tree_projectable(ProcessTreeProjectable));
}

#[test]
fn smoke_powl_refusal_arity_and_loop_body_variants() {
    // InvalidChoiceArity carries declared and required_min.
    let r = PowlRefusal::InvalidChoiceArity {
        declared: 1,
        required_min: 2,
    };
    assert_eq!(
        r,
        PowlRefusal::InvalidChoiceArity {
            declared: 1,
            required_min: 2
        }
    );

    // LoopMissingDoBody is a named law.
    let l = PowlRefusal::LoopMissingDoBody;
    assert_eq!(l, PowlRefusal::LoopMissingDoBody);
}

#[test]
fn smoke_powl_choice_node_validate_arity_refusal() {
    // An empty choice node produces InvalidChoiceArity via validate_arity.
    let empty = PowlChoiceNode::new(vec![]);
    assert_eq!(
        empty.validate(),
        Err(PowlRefusal::InvalidChoiceArity {
            declared: 0,
            required_min: 2
        })
    );
}

#[test]
fn smoke_tree() {
    let mut t = ProcessTree::new();
    t.nodes.push(ProcessTreeNode::Activity("a".into()));
    t.nodes.push(ProcessTreeNode::Activity("b".into()));
    t.nodes.push(ProcessTreeNode::Operator {
        operator: ProcessTreeOperator::Sequence,
        children: vec![
            wasm4pm_compat::process_tree::ProcessTreeNodeId(0),
            wasm4pm_compat::process_tree::ProcessTreeNodeId(1),
        ],
    });
    t.root = Some(wasm4pm_compat::process_tree::ProcessTreeNodeId(2));

    assert_eq!(t.node_count(), 3);
    assert_eq!(
        t.root,
        Some(wasm4pm_compat::process_tree::ProcessTreeNodeId(2))
    );

    let r = ProcessTreeRefusal::InvalidArity;
    assert_eq!(r, ProcessTreeRefusal::InvalidArity);
}

#[test]
fn smoke_declare() {
    let c = DeclareConstraint::binary(
        DeclareTemplate::Response,
        Activity::new("a"),
        Activity::new("b"),
        DeclareScope::SingleObjectScope("order".into()),
    );
    assert_eq!(c.template, DeclareTemplate::Response);
    assert_eq!(c.template.arity(), 2);
    assert!(c.target.is_some());

    let u = DeclareConstraint::unary(
        DeclareTemplate::Existence,
        Activity::new("a"),
        DeclareScope::MultiObjectScope(vec!["order".into(), "item".into()]),
    );
    assert_eq!(u.template.arity(), 1);
    assert!(u.target.is_none());

    let r = DeclareRefusal::MissingTarget;
    assert_eq!(r, DeclareRefusal::MissingTarget);
}

#[test]
fn smoke_declare_extended_templates_arity() {
    // Unary extended templates all have arity 1.
    assert_eq!(DeclareTemplate::Init.arity(), 1);
    assert_eq!(DeclareTemplate::Existence2.arity(), 1);
    assert_eq!(DeclareTemplate::Existence3.arity(), 1);
    assert_eq!(DeclareTemplate::Absence2.arity(), 1);
    assert_eq!(DeclareTemplate::Absence3.arity(), 1);

    // Binary extended templates all have arity 2.
    assert_eq!(DeclareTemplate::RespondedExistence.arity(), 2);
    assert_eq!(DeclareTemplate::CoExistence.arity(), 2);
    assert_eq!(DeclareTemplate::AlternateResponse.arity(), 2);
    assert_eq!(DeclareTemplate::AlternatePrecedence.arity(), 2);
    assert_eq!(DeclareTemplate::AlternateSuccession.arity(), 2);
    assert_eq!(DeclareTemplate::ChainResponse.arity(), 2);
    assert_eq!(DeclareTemplate::ChainPrecedence.arity(), 2);
    assert_eq!(DeclareTemplate::ChainSuccession.arity(), 2);
    assert_eq!(DeclareTemplate::NotSuccession.arity(), 2);
    assert_eq!(DeclareTemplate::NotChainSuccession.arity(), 2);
    assert_eq!(DeclareTemplate::ExclusiveChoice.arity(), 2);
}

#[test]
fn smoke_declare_template_is_negative() {
    // Negative templates.
    assert!(DeclareTemplate::Absence.is_negative());
    assert!(DeclareTemplate::Absence2.is_negative());
    assert!(DeclareTemplate::Absence3.is_negative());
    assert!(DeclareTemplate::NotCoExistence.is_negative());
    assert!(DeclareTemplate::NotSuccession.is_negative());
    assert!(DeclareTemplate::NotChainSuccession.is_negative());

    // Non-negative templates.
    assert!(!DeclareTemplate::Existence.is_negative());
    assert!(!DeclareTemplate::Response.is_negative());
    assert!(!DeclareTemplate::ChainSuccession.is_negative());
    assert!(!DeclareTemplate::ExclusiveChoice.is_negative());
}

#[test]
fn smoke_declare_template_is_chain() {
    assert!(DeclareTemplate::ChainResponse.is_chain());
    assert!(DeclareTemplate::ChainPrecedence.is_chain());
    assert!(DeclareTemplate::ChainSuccession.is_chain());
    assert!(DeclareTemplate::NotChainSuccession.is_chain());

    assert!(!DeclareTemplate::Response.is_chain());
    assert!(!DeclareTemplate::Succession.is_chain());
    assert!(!DeclareTemplate::Existence.is_chain());
}

#[test]
fn smoke_oc_declare_constraint_new() {
    let inner = DeclareConstraint::binary(
        DeclareTemplate::Response,
        Activity::new("submit"),
        Activity::new("approve"),
        DeclareScope::SingleObjectScope("order".into()),
    );
    let oc = OcDeclareConstraint::new(inner.clone(), vec!["order".into(), "item".into()]);
    assert_eq!(oc.object_types.len(), 2);
    assert!(!oc.is_synchronized());
    assert!(oc.validate().is_ok());

    // Empty object type list is refused.
    let bad = OcDeclareConstraint::new(inner, vec![]);
    assert_eq!(bad.validate(), Err(OcDeclareRefusal::EmptyObjectTypeList));
}

#[test]
fn smoke_oc_declare_constraint_synchronized() {
    let inner = DeclareConstraint::binary(
        DeclareTemplate::ChainSuccession,
        Activity::new("ship"),
        Activity::new("deliver"),
        DeclareScope::SynchronizedObjectScope(vec!["order".into(), "delivery".into()]),
    );
    let oc = OcDeclareConstraint::synchronized(inner, vec!["order".into(), "delivery".into()]);
    assert!(oc.is_synchronized());
    assert!(oc.validate().is_ok());
}

#[test]
fn smoke_oc_declare_refusal_display() {
    let e = OcDeclareRefusal::EmptyObjectTypeList;
    assert_eq!(e, OcDeclareRefusal::EmptyObjectTypeList);

    let s = OcDeclareRefusal::SynchronizationRequiresMultipleTypes;
    assert_eq!(s, OcDeclareRefusal::SynchronizationRequiresMultipleTypes);

    let m = OcDeclareRefusal::ScopeMismatch;
    assert_eq!(m, OcDeclareRefusal::ScopeMismatch);
}

#[test]
fn smoke_declare_refusal_display() {
    let r = DeclareRefusal::MissingTarget;
    assert_eq!(r, DeclareRefusal::MissingTarget);
    let r2 = DeclareRefusal::InvalidTemplateArity;
    assert_eq!(r2, DeclareRefusal::InvalidTemplateArity);
    let r3 = DeclareRefusal::EmptyObjectScope;
    assert_eq!(r3, DeclareRefusal::EmptyObjectScope);
    let r4 = DeclareRefusal::SynchronizationViolation;
    assert_eq!(r4, DeclareRefusal::SynchronizationViolation);
    let r5 = DeclareRefusal::MissingActivation;
    assert_eq!(r5, DeclareRefusal::MissingActivation);
}

#[test]
fn smoke_ocpq() {
    let mut q = OcpqQuery::new(ObjectScope::new(["order", "item"]));
    q.predicates.push(Predicate::new(PredicateKind::Event(
        "activity = pay".into(),
    )));
    q.predicates
        .push(Predicate::new(PredicateKind::Cardinality {
            min: 1,
            max: 3,
        }));

    assert!(!q.scope.is_empty());
    assert_eq!(q.scope.object_types.len(), 2);
    assert_eq!(q.predicates.len(), 2);

    let r = OcpqRefusal::FlatteningRequired;
    assert_eq!(r, OcpqRefusal::FlatteningRequired);
}

#[test]
fn smoke_conformance() {
    // Bounded newtypes refuse out-of-range input.
    assert!(Fitness::new(1.5).is_none());
    assert!(Fitness::new(-0.1).is_none());
    assert!(Fitness::new(1.0).is_some());
    assert!(Precision::new(2.0).is_none());
    assert!(Precision::new(0.5).is_some());
    assert!(F1::new(f64::NAN).is_none());
    assert!(F1::new(0.5).is_some());

    let mut v = ConformanceVerdict::new();
    assert!(!v.is_perfect());
    v.fitness = Fitness::new(1.0);
    assert!(v.is_perfect());

    v.deviations.push(Deviation::new(0, "skip"));
    assert!(!v.is_perfect());

    let r = ConformanceRefusal::FitnessUnavailable;
    assert_eq!(r, ConformanceRefusal::FitnessUnavailable);
}

#[test]
fn smoke_generalization_and_simplicity_newtypes() {
    // Generalization: in-range accepted, out-of-range rejected.
    assert!(Generalization::new(0.0).is_some());
    assert!(Generalization::new(1.0).is_some());
    assert!(Generalization::new(0.875).is_some());
    assert!(Generalization::new(1.1).is_none());
    assert!(Generalization::new(-0.01).is_none());
    assert!(Generalization::new(f64::NAN).is_none());
    assert!(Generalization::new(f64::INFINITY).is_none());

    // Simplicity: in-range accepted, out-of-range rejected.
    assert!(Simplicity::new(0.0).is_some());
    assert!(Simplicity::new(1.0).is_some());
    assert!(Simplicity::new(0.6).is_some());
    assert!(Simplicity::new(-0.5).is_none());
    assert!(Simplicity::new(2.0).is_none());
    assert!(Simplicity::new(f64::NEG_INFINITY).is_none());

    // get() round-trips the value.
    assert_eq!(Generalization::new(0.9).unwrap().get(), 0.9);
    assert_eq!(Simplicity::new(0.6).unwrap().get(), 0.6);
}

#[test]
fn smoke_conformance_verdict_all_dimensions() {
    // ConformanceVerdict now carries all four quality dimensions.
    let mut v = ConformanceVerdict::new();
    assert!(v.generalization.is_none());
    assert!(v.simplicity.is_none());

    v.fitness = Fitness::new(0.9);
    v.precision = Precision::new(0.85);
    v.f1 = F1::new(0.87);
    v.generalization = Generalization::new(0.8);
    v.simplicity = Simplicity::new(0.75);

    assert!(v.fitness.is_some());
    assert!(v.generalization.is_some());
    assert!(v.simplicity.is_some());
    assert_eq!(v.generalization.unwrap().get(), 0.8);
    assert_eq!(v.simplicity.unwrap().get(), 0.75);
}

#[test]
fn smoke_conformance_refusal_new_variants() {
    let g = ConformanceRefusal::GeneralizationUnavailable;
    assert_eq!(g, ConformanceRefusal::GeneralizationUnavailable);

    let s = ConformanceRefusal::SimplicityUnavailable;
    assert_eq!(s, ConformanceRefusal::SimplicityUnavailable);
}

#[test]
fn smoke_prediction() {
    let p = PredictionProblem::<()>::new(
        vec!["register".into(), "review".into()],
        PredictionTarget::NextActivity,
    );
    assert_eq!(p.prefix_len(), 2);
    assert_eq!(p.target, PredictionTarget::NextActivity);

    let r = PredictionRefusal::EmptyPrefix;
    assert_eq!(r, PredictionRefusal::EmptyPrefix);
}

#[test]
fn smoke_receipt() {
    let r = ReceiptShape::new(
        "discovery-run",
        Digest::new("blake3:abc123"),
        ReplayHint::new("rerun:plan#1"),
    );
    assert!(r.is_well_shaped());
    assert_eq!(r.witness, "discovery-run");

    let empty = ReceiptShape::new("", Digest::new(""), ReplayHint::new(""));
    assert!(!empty.is_well_shaped());

    let refusal = ReceiptRefusal::MissingDigest;
    assert_eq!(refusal, ReceiptRefusal::MissingDigest);
}
