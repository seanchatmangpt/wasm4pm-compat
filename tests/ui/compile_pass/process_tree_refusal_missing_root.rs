// COMPILE-PASS: ProcessTreeRefusal::MissingRoot when nodes present but no root.
//
// Law: a non-empty process tree without a declared root is inadmissible.
// admit_shape() refuses with MissingRoot, naming the structural law.
use wasm4pm_compat::process_tree::{ProcessTree, ProcessTreeNode, ProcessTreeRefusal};

fn main() {
    let mut t = ProcessTree::new();
    t.nodes.push(ProcessTreeNode::Activity("orphan".into()));
    // No root assigned.
    assert_eq!(t.admit_shape(), Err(ProcessTreeRefusal::MissingRoot));
}
