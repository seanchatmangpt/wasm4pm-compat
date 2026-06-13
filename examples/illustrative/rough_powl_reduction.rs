//! Example: Rough POWL model reduction
//!
//! Demonstrates a simple rule-based reducer for POWL models.
//! Specifically, it merges consecutive silent transitions within a Partial Order.
//!
//! Run: cargo run --example rough_powl_reduction

use wasm4pm_compat::powl::{OrderEdge, Powl, PowlNode, PowlNodeId, PowlNodeKind};

fn main() {
    println!("--- Rough POWL Reduction Simulator ---");

    // 1. Create a complex Powl model
    // Structure: A -> S1 -> S2 -> B -> (C XOR D) -> S3 -> S4 -> E
    // where S are Silent transitions.
    let mut model = Powl::new();

    // Activities
    let a = add_atom(&mut model, "A");
    let s1 = add_silent(&mut model);
    let s2 = add_silent(&mut model);
    let b = add_atom(&mut model, "B");
    let c = add_atom(&mut model, "C");
    let d = add_atom(&mut model, "D");
    let xor = add_choice(&mut model, vec![c, d]);
    let s3 = add_silent(&mut model);
    let s4 = add_silent(&mut model);
    let e = add_atom(&mut model, "E");

    // Root Partial Order containing everything in sequence
    let root_nodes = vec![a, s1, s2, b, xor, s3, s4, e];
    let root_id = PowlNodeId(model.nodes.len());
    model.nodes.push(PowlNode::new(
        root_id,
        PowlNodeKind::PartialOrder(root_nodes.clone()),
    ));
    model.root = Some(root_id);

    // Sequence edges: A -> S1 -> S2 -> B -> XOR -> S3 -> S4 -> E
    model.edges.push(OrderEdge::new(a, s1));
    model.edges.push(OrderEdge::new(s1, s2));
    model.edges.push(OrderEdge::new(s2, b));
    model.edges.push(OrderEdge::new(b, xor));
    model.edges.push(OrderEdge::new(xor, s3));
    model.edges.push(OrderEdge::new(s3, s4));
    model.edges.push(OrderEdge::new(s4, e));

    println!("Initial model node count: {}", model.nodes.len());
    println!("Initial model edge count: {}", model.edges.len());

    // Verify initial structure
    match model.validate() {
        Ok(_) => println!("Initial model is valid POWL."),
        Err(e) => println!("Initial model is INVALID: {}", e),
    }

    // 2. Implement a simple rule-based reducer
    println!("\nReducing consecutive silent transitions...");
    let reduced = reduce_consecutive_silents(model);

    // 3. Verify final structure
    println!("Reduced model node count: {}", reduced.nodes.len());
    println!("Reduced model edge count: {}", reduced.edges.len());

    match reduced.validate() {
        Ok(_) => println!("Reduced model is valid POWL."),
        Err(e) => {
            println!("Reduced model is INVALID: {}", e);
            // Even if it's invalid due to some node references being broken in this "rough" implementation,
            // we'll print it out.
        }
    }

    println!("\nFinal structure summary:");
    for node in &reduced.nodes {
        println!("  Node {:?}: {:?}", node.id, node.kind);
    }
    println!("Final edges:");
    for edge in &reduced.edges {
        println!("  {:?} -> {:?}", edge.from, edge.to);
    }
}

fn add_atom(model: &mut Powl, label: &str) -> PowlNodeId {
    let id = PowlNodeId(model.nodes.len());
    model
        .nodes
        .push(PowlNode::new(id, PowlNodeKind::Atom(label.into())));
    id
}

fn add_silent(model: &mut Powl) -> PowlNodeId {
    let id = PowlNodeId(model.nodes.len());
    model.nodes.push(PowlNode::new(id, PowlNodeKind::Silent));
    id
}

fn add_choice(model: &mut Powl, branches: Vec<PowlNodeId>) -> PowlNodeId {
    let id = PowlNodeId(model.nodes.len());
    model
        .nodes
        .push(PowlNode::new(id, PowlNodeKind::Choice(branches)));
    id
}

/// A "rough" reducer that merges S1 -> S2 into S1 if both are Silent.
fn reduce_consecutive_silents(mut model: Powl) -> Powl {
    let mut changed = true;
    while changed {
        changed = false;

        let mut to_merge = None;

        // Find a candidate: S1 -> S2 where both are Silent
        for edge in &model.edges {
            let from_kind = get_kind(&model, edge.from);
            let to_kind = get_kind(&model, edge.to);

            if matches!(from_kind, Some(PowlNodeKind::Silent))
                && matches!(to_kind, Some(PowlNodeKind::Silent))
            {
                // Check if they are strictly consecutive in the same PartialOrder
                // For simplicity in this "rough" version, we just check if they are both in the root PartialOrder
                // and S1 has only one output (to S2) and S2 has only one input (from S1).

                let from_out_degree = model.edges.iter().filter(|e| e.from == edge.from).count();
                let to_in_degree = model.edges.iter().filter(|e| e.to == edge.to).count();

                if from_out_degree == 1 && to_in_degree == 1 {
                    to_merge = Some((edge.from, edge.to));
                    break;
                }
            }
        }

        if let Some((s1, s2)) = to_merge {
            println!("Merging {:?} and {:?}", s1, s2);

            // Redirect all edges from S2 to come from S1 instead
            let mut new_edges = Vec::new();
            for edge in model.edges {
                if edge.from == s1 && edge.to == s2 {
                    // Remove this edge
                    continue;
                }
                if edge.from == s2 {
                    new_edges.push(OrderEdge::new(s1, edge.to));
                } else if edge.to == s2 {
                    // This shouldn't happen if in_degree == 1 and we are merging s1 -> s2
                    // but for completeness:
                    new_edges.push(OrderEdge::new(edge.from, s1));
                } else {
                    new_edges.push(edge);
                }
            }
            model.edges = new_edges;

            // Remove s2 from any PartialOrder or Choice nodes
            for node in &mut model.nodes {
                match &mut node.kind {
                    PowlNodeKind::PartialOrder(children) => {
                        children.retain(|&id| id != s2);
                    }
                    PowlNodeKind::Choice(branches) => {
                        branches.retain(|&id| id != s2);
                    }
                    _ => {}
                }
            }

            // Remove s2 from model nodes
            model.nodes.retain(|n| n.id != s2);

            changed = true;
        }
    }
    model
}

fn get_kind(model: &Powl, id: PowlNodeId) -> Option<&PowlNodeKind> {
    model.nodes.iter().find(|n| n.id == id).map(|n| &n.kind)
}
