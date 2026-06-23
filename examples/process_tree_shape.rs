//! Process tree shape — block-structured process model vocabulary.
//!
//! Demonstrates the `process_tree` module:
//!
//! - [`operator_minimum_arity`] / [`operator_maximum_arity`] — const arity tables
//! - [`TypedLoopNode<Children, ARITY>`] — compile-time loop arity law (ARITY == 2)
//! - [`TypedXorNode`] / [`TypedAndNode`] / [`TypedSeqNode`] / [`TypedOrNode`] — arity >= 2
//! - [`ProcessTreeNodeId`] — zero-cost arena index
//! - [`ProcessTreeOperator`] — 6 operator variants
//! - [`ProcessTreeNode`] — Activity | Operator
//! - [`ProcessTree`] — arena + root + `admit_shape()`
//! - [`ProcessTreeRefusal`] — 9 named structural laws
//!
//! **This crate does NOT discover or replay.** Graduate to `wasm4pm` for mining.
//!
//! Doc reference: `src/process_tree.rs`

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use wasm4pm_compat::law::ProcessTreeOperatorKind;
use wasm4pm_compat::process_tree::{
    operator_maximum_arity, operator_minimum_arity, ProcessTree, ProcessTreeNode,
    ProcessTreeNodeId, ProcessTreeOperator, ProcessTreeRefusal, TypedAndNode, TypedLoopNode,
    TypedOrNode, TypedSeqNode, TypedXorNode,
};

fn main() {
    println!("=== Process tree shape (structure only) ===\n");

    // ── Part 1: Arity const tables ────────────────────────────────────────────
    println!("Part 1: operator_minimum_arity / operator_maximum_arity");

    let cases = [
        (ProcessTreeOperatorKind::Sequence, 2, usize::MAX),
        (ProcessTreeOperatorKind::Xor, 2, usize::MAX),
        (ProcessTreeOperatorKind::Parallel, 2, usize::MAX),
        (ProcessTreeOperatorKind::Loop, 2, 2),
        (ProcessTreeOperatorKind::Silent, 0, 0),
        (ProcessTreeOperatorKind::Or, 2, usize::MAX),
    ];
    for (kind, expected_min, expected_max) in &cases {
        assert_eq!(
            operator_minimum_arity(*kind),
            *expected_min,
            "min arity {:?}",
            kind
        );
        assert_eq!(
            operator_maximum_arity(*kind),
            *expected_max,
            "max arity {:?}",
            kind
        );
        println!(
            "  ✓ {:?}: min={}, max={}",
            kind,
            expected_min,
            if *expected_max == usize::MAX {
                "∞".into()
            } else {
                expected_max.to_string()
            }
        );
    }

    // ── Part 2: Typed operator nodes — compile-time arity law ─────────────────
    println!("\nPart 2: Typed operator nodes (arity enforced by type law)");

    // TypedLoopNode: ARITY must == 2 (Leemans do-body + redo-branch)
    let loop_node: TypedLoopNode<[&str; 2], 2> = TypedLoopNode::new(["do-body", "redo-branch"]);
    assert_eq!(loop_node.children, ["do-body", "redo-branch"]);
    println!(
        "  ✓ TypedLoopNode<ARITY=2>: children={:?}",
        loop_node.children
    );

    // TypedXorNode: ARITY must >= 2
    let xor_node: TypedXorNode<[&str; 3], 3> = TypedXorNode::new(["approve", "reject", "delegate"]);
    assert_eq!(xor_node.children.len(), 3);
    println!(
        "  ✓ TypedXorNode<ARITY=3>: {} branches",
        xor_node.children.len()
    );

    // TypedAndNode: ARITY must >= 2
    let and_node: TypedAndNode<[&str; 2], 2> = TypedAndNode::new(["audit", "notify"]);
    assert_eq!(and_node.children[0], "audit");
    println!("  ✓ TypedAndNode<ARITY=2>: {:?}", and_node.children);

    // TypedSeqNode: ARITY must >= 2
    let seq_node: TypedSeqNode<[&str; 3], 3> = TypedSeqNode::new(["register", "review", "close"]);
    assert_eq!(seq_node.children[2], "close");
    println!("  ✓ TypedSeqNode<ARITY=3>: last={}", seq_node.children[2]);

    // TypedOrNode: ARITY must >= 2
    let or_node: TypedOrNode<[&str; 2], 2> = TypedOrNode::new(["path-a", "path-b"]);
    assert_eq!(or_node.children[1], "path-b");
    println!("  ✓ TypedOrNode<ARITY=2>: {:?}", or_node.children);

    // ── Part 3: ProcessTreeNodeId ─────────────────────────────────────────────
    println!("\nPart 3: ProcessTreeNodeId zero-cost arena index");

    let id0 = ProcessTreeNodeId(0);
    let id1 = ProcessTreeNodeId(1);
    assert_eq!(id0.0, 0);
    assert!(id0 < id1);
    assert_eq!(
        core::mem::size_of::<ProcessTreeNodeId>(),
        core::mem::size_of::<usize>()
    );
    println!("  ✓ ProcessTreeNodeId(0) < ProcessTreeNodeId(1)");
    println!("  ✓ size_of::<ProcessTreeNodeId>() == size_of::<usize>() (zero-cost repr)");

    // ── Part 4: ProcessTree::admit_shape() — lawful tree ─────────────────────
    println!("\nPart 4: ProcessTree::admit_shape() — lawful Sequence(a, b)");

    let mut tree = ProcessTree::new();
    assert!(tree.root.is_none());
    assert_eq!(tree.node_count(), 0);

    tree.nodes
        .push(ProcessTreeNode::Activity("register".into())); // id 0
    tree.nodes.push(ProcessTreeNode::Activity("close".into())); // id 1
    tree.nodes.push(ProcessTreeNode::Operator {
        operator: ProcessTreeOperator::Sequence,
        children: vec![ProcessTreeNodeId(0), ProcessTreeNodeId(1)],
    }); // id 2
    tree.root = Some(ProcessTreeNodeId(2));

    assert_eq!(tree.node_count(), 3);
    assert_eq!(tree.admit_shape(), Ok(()));
    println!("  ✓ Sequence(register, close): node_count=3, admit_shape()=Ok");

    // ── Part 5: ProcessTree refusals — named structural laws ──────────────────
    println!("\nPart 5: ProcessTreeRefusal named laws");

    // MissingRoot — nodes present but no root
    let mut no_root = ProcessTree::new();
    no_root.nodes.push(ProcessTreeNode::Activity("a".into()));
    assert_eq!(no_root.admit_shape(), Err(ProcessTreeRefusal::MissingRoot));
    println!("  ✓ nodes + no root → MissingRoot");

    // DanglingNodeReference — child id out of bounds
    let mut dangling = ProcessTree::new();
    dangling.nodes.push(ProcessTreeNode::Activity("a".into()));
    dangling.nodes.push(ProcessTreeNode::Operator {
        operator: ProcessTreeOperator::Sequence,
        children: vec![ProcessTreeNodeId(0), ProcessTreeNodeId(99)], // 99 doesn't exist
    });
    dangling.root = Some(ProcessTreeNodeId(1));
    assert_eq!(
        dangling.admit_shape(),
        Err(ProcessTreeRefusal::DanglingNodeReference)
    );
    println!("  ✓ child id 99 out of bounds → DanglingNodeReference");

    // TauLeafWithChildren — Silent with children
    let mut tau_bad = ProcessTree::new();
    tau_bad.nodes.push(ProcessTreeNode::Activity("a".into()));
    tau_bad.nodes.push(ProcessTreeNode::Operator {
        operator: ProcessTreeOperator::Silent,
        children: vec![ProcessTreeNodeId(0)], // tau can't have children
    });
    tau_bad.root = Some(ProcessTreeNodeId(1));
    assert_eq!(
        tau_bad.admit_shape(),
        Err(ProcessTreeRefusal::TauLeafWithChildren)
    );
    println!("  ✓ Silent with children → TauLeafWithChildren");

    // BelowMinimumArity — Sequence with 1 child
    let mut below_arity = ProcessTree::new();
    below_arity
        .nodes
        .push(ProcessTreeNode::Activity("a".into()));
    below_arity.nodes.push(ProcessTreeNode::Operator {
        operator: ProcessTreeOperator::Sequence,
        children: vec![ProcessTreeNodeId(0)], // needs >= 2
    });
    below_arity.root = Some(ProcessTreeNodeId(1));
    assert_eq!(
        below_arity.admit_shape(),
        Err(ProcessTreeRefusal::BelowMinimumArity)
    );
    println!("  ✓ Sequence with 1 child → BelowMinimumArity");

    // InvalidArity — Loop with 3 children
    let mut loop_bad = ProcessTree::new();
    loop_bad.nodes.push(ProcessTreeNode::Activity("a".into()));
    loop_bad.nodes.push(ProcessTreeNode::Activity("b".into()));
    loop_bad.nodes.push(ProcessTreeNode::Activity("c".into()));
    loop_bad.nodes.push(ProcessTreeNode::Operator {
        operator: ProcessTreeOperator::Loop,
        children: vec![
            ProcessTreeNodeId(0),
            ProcessTreeNodeId(1),
            ProcessTreeNodeId(2),
        ],
    });
    loop_bad.root = Some(ProcessTreeNodeId(3));
    assert_eq!(
        loop_bad.admit_shape(),
        Err(ProcessTreeRefusal::InvalidArity)
    );
    println!("  ✓ Loop with 3 children → InvalidArity");

    // Display strings contain the law name
    let refusals: &[ProcessTreeRefusal] = &[
        ProcessTreeRefusal::MissingRoot,
        ProcessTreeRefusal::TauLeafWithChildren,
        ProcessTreeRefusal::BelowMinimumArity,
        ProcessTreeRefusal::InvalidArity,
        ProcessTreeRefusal::DanglingNodeReference,
        ProcessTreeRefusal::CycleDetected,
        ProcessTreeRefusal::UnsupportedProjection,
        ProcessTreeRefusal::LanguageMismatch,
        ProcessTreeRefusal::InvalidLoop,
    ];
    for r in refusals {
        let s = format!("{r}");
        assert!(
            s.starts_with("process tree refused:"),
            "Display prefix wrong: {}",
            s
        );
    }
    println!("  ✓ All 9 ProcessTreeRefusal variants have Display");

    // ── Part 6: ProcessTreeOperator — 6 variants ──────────────────────────────
    println!("\nPart 6: ProcessTreeOperator 6 variants");

    let ops = [
        ProcessTreeOperator::Sequence,
        ProcessTreeOperator::Xor,
        ProcessTreeOperator::Parallel,
        ProcessTreeOperator::Loop,
        ProcessTreeOperator::Silent,
        ProcessTreeOperator::Or,
    ];
    for op in &ops {
        println!("  ✓ {:?}", op);
    }

    println!("\n=== All assertions passed — process_tree module surface is witnessed ===");
    println!("  Covered: operator_minimum/maximum_arity (6 operator kinds),");
    println!("           TypedLoopNode/XorNode/AndNode/SeqNode/OrNode (arity law),");
    println!("           ProcessTreeNodeId (zero-cost, ordered), ProcessTreeOperator (6),");
    println!("           ProcessTree::admit_shape() (ok + 5 named refusal laws),");
    println!("           ProcessTreeRefusal (9 laws with Display).");
    println!("  Structure only — no discovery, no replay, no conformance.");
    println!("  Graduate to wasm4pm for: Inductive Miner, tree replay, language equivalence.");
}
