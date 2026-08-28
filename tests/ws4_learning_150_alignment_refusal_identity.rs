use wasm4pm_compat::alignment::AlignmentRefusal;

#[test]
fn alignment_refusals_have_stable_named_diagnostics() {
    assert_eq!(
        AlignmentRefusal::UngroundedAlignment.to_string(),
        "alignment refusal: UngroundedAlignment"
    );
    assert_eq!(
        AlignmentRefusal::EmptyAlignment.to_string(),
        "alignment refusal: EmptyAlignment"
    );
}
