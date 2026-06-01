// COMPILE-PASS: PowlNodeKind::Loop construction — body and optional redo.
//
// Law: Kourani et al. (2026) §3 — a POWL loop L(M₁, M₂) has a mandatory body
// and an optional redo. PowlNodeKind::Loop carries both fields structurally.
use wasm4pm_compat::powl::{PowlNode, PowlNodeId, PowlNodeKind, Loop};
use core::marker::PhantomData;

fn main() {
    let body_id = PowlNodeId(0);
    let redo_id = PowlNodeId(1);

    // Loop with both body and redo.
    let loop_with_redo: PowlNode<Loop> = PowlNode {
        id: PowlNodeId(2),
        kind: PowlNodeKind::Loop { body: body_id, redo: Some(redo_id) },
        witness: PhantomData,
    };
    assert!(matches!(loop_with_redo.kind, PowlNodeKind::Loop { redo: Some(_), .. }));

    // Loop with body only (no redo — executes once).
    let loop_no_redo: PowlNode<Loop> = PowlNode {
        id: PowlNodeId(3),
        kind: PowlNodeKind::Loop { body: body_id, redo: None },
        witness: PhantomData,
    };
    assert!(matches!(loop_no_redo.kind, PowlNodeKind::Loop { redo: None, .. }));
}
