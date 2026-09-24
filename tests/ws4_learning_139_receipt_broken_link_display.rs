use wasm4pm_compat::receipt::ReceiptRefusal;

#[test]
fn broken_chain_link_diagnostic_binds_exact_index() {
    assert_eq!(
        ReceiptRefusal::BrokenChainLink(7).to_string(),
        "receipt refused: BrokenChainLink at index 7"
    );
}
