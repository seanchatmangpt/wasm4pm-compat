use wasm4pm_compat::receipt::ReceiptRefusal;

#[test]
fn missing_subject_refusal_has_stable_diagnostic_identity() {
    assert_eq!(
        ReceiptRefusal::MissingSubject.to_string(),
        "receipt refused: MissingSubject"
    );
}
