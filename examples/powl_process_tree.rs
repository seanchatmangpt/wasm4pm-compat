//! Example: POWL and process tree shapes
//!
//! Demonstrates the structural vocabulary of POWL and process trees in
//! wasm4pm-compat: witness markers, typed arity-enforced nodes, partial order
//! construction, choice and loop operators, and the sealed `TreeProjectable`
//! gate. All shapes are structure-only — no execution happens here.
//!
//! Run: cargo run --example powl_process_tree

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![allow(dead_code)]

use wasm4pm_compat::powl::{
    assert_tree_projectable, AcyclicPartialOrder, ChoiceGraphEdge, ChoiceGraphMarker, OrderEdge,
    PartialOrder, Powl, PowlChoiceNode, PowlNode, PowlNodeId, PowlNodeKind, PowlRefusal,
    ProcessTreeProjectable, TypedPowlLoopNode,
};
use wasm4pm_compat::process_tree::{
    ProcessTree, ProcessTreeNode, ProcessTreeNodeId, ProcessTreeOperator, TypedAndNode,
    TypedLoopNode, TypedSeqNode, TypedXorNode,
};

fn main() {
    demo_process_tree_projectable_gate();
    demo_powl_partial_order_witness();
    demo_powl_choice_construction();
    demo_typed_loop_arity();
    demo_process_tree_operators();

    // TypedLoopNode<_, 3> or TypedPowlLoopNode<_, 3> do NOT compile.
    // The const-generic bound `Require<{ ARITY == 2 }>: IsTrue` is violated
    // at compile time — arity 3 is refused before any binary is produced.
    //
    // Uncomment to see the compile error:
    //   let _: TypedLoopNode<(), 3> = TypedLoopNode::new(());
    //   let _: TypedPowlLoopNode<(), 3> = TypedPowlLoopNode::new(());

    println!("All POWL and process tree shape demonstrations passed.");
}

// ── ProcessTreeProjectable sealed gate ──────────────────────────────────────

/// `ProcessTreeProjectable` is a sealed marker: only POWL fragments that are
/// structurally reducible to a block-structured process tree carry this witness.
/// `assert_tree_projectable` accepts only `ProcessTreeProjectable` — passing
/// `ExceedsProcessTree` would be a compile error.
fn demo_process_tree_projectable_gate() {
    // A POWL fragment tagged as tree-projectable passes the structural gate.
    let marker = ProcessTreeProjectable;
    let passed = assert_tree_projectable(marker);
    assert!(
        passed,
        "ProcessTreeProjectable must pass the tree-projectable gate"
    );

    // The gate is sealed: ExceedsProcessTree does NOT satisfy TreeProjectable.
    // Uncommenting the next line would fail to compile:
    //   assert_tree_projectable(wasm4pm_compat::powl::ExceedsProcessTree);

    println!("[1] ProcessTreeProjectable gate: ok");
}

// ── POWL partial order witness ───────────────────────────────────────────────

/// A partial order node uses `PartialOrder` as its witness type.
/// `AcyclicPartialOrder` is the stronger claim: acyclicity has been asserted.
fn demo_powl_partial_order_witness() {
    // Build a simple two-activity partial order: A must precede B.
    let node_a = PowlNodeId(0);
    let node_b = PowlNodeId(1);

    let po_node: PowlNode<PartialOrder> = PowlNode::new(
        PowlNodeId(2),
        PowlNodeKind::PartialOrder(vec![node_a, node_b]),
    );
    assert_eq!(po_node.id, PowlNodeId(2));

    // Record the precedence edge A → B.
    let edge = OrderEdge::new(node_a, node_b);
    assert_eq!(edge.from, node_a);
    assert_eq!(edge.to, node_b);

    // An AcyclicPartialOrder marker carries the stronger acyclicity assertion.
    // assert_acyclic only accepts AcyclicPartialOrder — PartialOrder alone fails.
    let acyclic = AcyclicPartialOrder;
    let _ = acyclic; // marker is zero-cost; no runtime cost

    // Assemble a minimal Powl model.
    // Powl::nodes is Vec<PowlNode> (i.e. PowlNode<()>): push unwitnessed nodes
    // into the collection. Witnessed nodes (e.g. PowlNode<PartialOrder>) are
    // used for type-checked construction, then stored as plain PowlNode.
    let mut model = Powl::new();
    model
        .nodes
        .push(PowlNode::new(node_a, PowlNodeKind::Atom("A".into())));
    model
        .nodes
        .push(PowlNode::new(node_b, PowlNodeKind::Atom("B".into())));
    // Store the partial-order node id and kind, dropping the witness PhantomData.
    model.nodes.push(PowlNode::new(po_node.id, po_node.kind));
    model.edges.push(edge);
    model.root = Some(PowlNodeId(2));
    assert_eq!(model.node_count(), 3);

    println!(
        "[2] POWL partial order witness: ok (nodes={}, edges={})",
        model.node_count(),
        model.edges.len()
    );
}

// ── POWL choice construction ─────────────────────────────────────────────────

/// `PowlChoiceNode` carries the branch list for an exclusive-choice (XOR) node.
/// A well-formed choice requires at least two branches; a single-branch choice
/// is refused as `PowlRefusal::InvalidChoice`.
fn demo_powl_choice_construction() {
    // Well-formed choice: two branches.
    let choice = PowlChoiceNode::new(vec![PowlNodeId(0), PowlNodeId(1)]);
    assert!(choice.is_well_formed());
    assert_eq!(choice.branch_count(), 2);
    assert!(choice.validate().is_ok());

    // Ill-formed choice: single branch is refused.
    let bad = PowlChoiceNode::new(vec![PowlNodeId(0)]);
    assert!(!bad.is_well_formed());
    assert_eq!(bad.validate(), Err(PowlRefusal::InvalidChoice));

    // A PowlNode carrying the ChoiceGraphMarker witness.
    let choice_node: PowlNode<ChoiceGraphMarker> = PowlNode::new(
        PowlNodeId(2),
        PowlNodeKind::ChoiceGraph {
            nodes: vec![PowlNodeId(0), PowlNodeId(1)],
            edges: vec![ChoiceGraphEdge::new(PowlNodeId(0), PowlNodeId(1))],
        },
    );
    assert_eq!(choice_node.id, PowlNodeId(2));

    println!("[3] POWL choice construction: ok");
}

// ── Typed loop arity enforcement ─────────────────────────────────────────────

/// `TypedLoopNode<_, 2>` compiles; arity 3 would be a compile error.
/// The same law applies in the POWL domain via `TypedPowlLoopNode<_, 2>`.
fn demo_typed_loop_arity() {
    // Process-tree domain: Loop(do, redo) — exactly 2 children.
    let loop_node: TypedLoopNode<[&str; 2], 2> = TypedLoopNode::new(["do_body", "redo_branch"]);
    assert_eq!(loop_node.children[0], "do_body");
    assert_eq!(loop_node.children[1], "redo_branch");

    // POWL domain: same arity law for TypedPowlLoopNode.
    let powl_loop: TypedPowlLoopNode<[&str; 2], 2> =
        TypedPowlLoopNode::new(["do_body", "redo_branch"]);
    assert_eq!(powl_loop.children[0], "do_body");

    // A PowlNode carrying the ChoiceGraphMarker witness is structurally distinct.
    let loop_kind_node: PowlNode<ChoiceGraphMarker> = PowlNode::new(
        PowlNodeId(10),
        PowlNodeKind::ChoiceGraph {
            nodes: vec![PowlNodeId(8), PowlNodeId(9)],
            edges: vec![ChoiceGraphEdge::new(PowlNodeId(8), PowlNodeId(9))],
        },
    );
    assert_eq!(loop_kind_node.id, PowlNodeId(10));

    // TypedLoopNode<_, 3> would fail to compile — arity 3 violates `ARITY == 2`.
    // TypedPowlLoopNode<_, 3> likewise does NOT compile.

    println!(
        "[4] Typed loop arity enforcement: ok (loop_node.children={:?})",
        loop_node.children
    );
}

// ── ProcessOperator: Xor, And, Seq ───────────────────────────────────────────

/// Process tree operator nodes use const-generic typed wrappers enforcing arity
/// at compile time, and `ProcessTreeOperator` as the structural label.
fn demo_process_tree_operators() {
    // XOR: exclusive choice — at least 2 branches.
    let xor: TypedXorNode<[&str; 2], 2> = TypedXorNode::new(["branch_a", "branch_b"]);
    assert_eq!(xor.children.len(), 2);

    // AND (parallel): at least 2 concurrent branches.
    let and: TypedAndNode<[&str; 3], 3> = TypedAndNode::new(["left", "mid", "right"]);
    assert_eq!(and.children.len(), 3);

    // SEQ (sequence): at least 2 steps.
    let seq: TypedSeqNode<[&str; 2], 2> = TypedSeqNode::new(["first", "second"]);
    assert_eq!(seq.children[0], "first");

    // Build a well-formed ProcessTree: Seq(Activity("a"), Activity("b")).
    let mut tree = ProcessTree::new();
    tree.nodes.push(ProcessTreeNode::Activity("a".into()));
    tree.nodes.push(ProcessTreeNode::Activity("b".into()));
    tree.nodes.push(ProcessTreeNode::Operator {
        operator: ProcessTreeOperator::Sequence,
        children: vec![ProcessTreeNodeId(0), ProcessTreeNodeId(1)],
    });
    tree.root = Some(ProcessTreeNodeId(2));
    assert_eq!(tree.admit_shape(), Ok(()));

    // XOR process-tree operator label (structural, not executed).
    let xor_op = ProcessTreeOperator::Xor;
    let and_op = ProcessTreeOperator::Parallel;
    let seq_op = ProcessTreeOperator::Sequence;
    assert_ne!(xor_op, and_op);
    assert_ne!(and_op, seq_op);

    println!(
        "[5] ProcessOperator shapes: Xor={:?}, And={:?}, Seq={:?} — tree admit_shape=ok",
        xor_op, and_op, seq_op
    );
}
