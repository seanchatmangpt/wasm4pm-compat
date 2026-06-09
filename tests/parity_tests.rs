use wasm4pm_compat::multiperspective::ParityComparer;

#[test]
fn test_parity_comparer_close() {
    ParityComparer::assert_epsilon_close(1.0000001, 1.0);
}

#[test]
#[should_panic(expected = "Parity violation")]
fn test_parity_comparer_panic() {
    ParityComparer::assert_epsilon_close(1.00001, 1.0);
}
