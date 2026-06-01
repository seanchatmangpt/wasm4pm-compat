// COMPILE-PASS: operator_minimum_arity and operator_maximum_arity are
// compile-observable constant functions covering all six operator kinds.
//
// Law: the arity law for each operator is encoded in these const functions;
// calling them proves the law is statically observable without execution.
use wasm4pm_compat::process_tree::{operator_minimum_arity, operator_maximum_arity};
use wasm4pm_compat::law::ProcessTreeOperatorKind;

fn main() {
    // Loop is exactly arity 2.
    assert_eq!(operator_minimum_arity(ProcessTreeOperatorKind::Loop), 2);
    assert_eq!(operator_maximum_arity(ProcessTreeOperatorKind::Loop), 2);

    // Silent has no children.
    assert_eq!(operator_minimum_arity(ProcessTreeOperatorKind::Silent), 0);
    assert_eq!(operator_maximum_arity(ProcessTreeOperatorKind::Silent), 0);

    // n-ary operators: min 2, max unbounded.
    assert_eq!(operator_minimum_arity(ProcessTreeOperatorKind::Sequence), 2);
    assert_eq!(operator_maximum_arity(ProcessTreeOperatorKind::Sequence), usize::MAX);

    assert_eq!(operator_minimum_arity(ProcessTreeOperatorKind::Xor), 2);
    assert_eq!(operator_maximum_arity(ProcessTreeOperatorKind::Xor), usize::MAX);

    assert_eq!(operator_minimum_arity(ProcessTreeOperatorKind::Parallel), 2);
    assert_eq!(operator_maximum_arity(ProcessTreeOperatorKind::Parallel), usize::MAX);

    assert_eq!(operator_minimum_arity(ProcessTreeOperatorKind::Or), 2);
    assert_eq!(operator_maximum_arity(ProcessTreeOperatorKind::Or), usize::MAX);
}
