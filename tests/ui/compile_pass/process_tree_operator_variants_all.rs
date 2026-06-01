// COMPILE-PASS: ProcessTreeOperator closed set — all six variants are reachable.
//
// Law: the ProcessTreeOperator enum is the closed set of block-structured
// process-tree operators. This fixture proves all six variants construct and
// can be matched exhaustively (no hidden arms).
use wasm4pm_compat::process_tree::ProcessTreeOperator;

fn operator_name(op: ProcessTreeOperator) -> &'static str {
    match op {
        ProcessTreeOperator::Sequence => "Sequence",
        ProcessTreeOperator::Xor      => "Xor",
        ProcessTreeOperator::Parallel => "Parallel",
        ProcessTreeOperator::Loop     => "Loop",
        ProcessTreeOperator::Silent   => "Silent",
        ProcessTreeOperator::Or       => "Or",
    }
}

fn main() {
    assert_eq!(operator_name(ProcessTreeOperator::Sequence), "Sequence");
    assert_eq!(operator_name(ProcessTreeOperator::Xor),      "Xor");
    assert_eq!(operator_name(ProcessTreeOperator::Parallel), "Parallel");
    assert_eq!(operator_name(ProcessTreeOperator::Loop),     "Loop");
    assert_eq!(operator_name(ProcessTreeOperator::Silent),   "Silent");
    assert_eq!(operator_name(ProcessTreeOperator::Or),       "Or");
}
