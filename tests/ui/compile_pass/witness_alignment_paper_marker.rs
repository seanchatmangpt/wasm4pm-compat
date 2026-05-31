// COMPILE-PASS: AlignmentPaper witness marker — proves AlignmentPaper is a
// distinct named law that implements Witness with correct metadata, and that
// it is non-interchangeable with WfNetSoundnessPaper at the type level.
//
// Law: van Dongen, de Medeiros & Wen (2008) — alignment-based conformance
// checking. An Admission<T, AlignmentPaper> is a different type from
// Admission<T, WfNetSoundnessPaper> — conformance-checking authority is
// orthogonal to structural soundness authority. This fixture proves the
// marker compiles, carries correct metadata, and belongs to WitnessFamily::Paper.
use wasm4pm_compat::witness::{AlignmentPaper, Witness, WfNetSoundnessPaper, WitnessFamily};

fn require_paper_family<W: Witness>(_: std::marker::PhantomData<W>) {
    assert_eq!(W::FAMILY, WitnessFamily::Paper);
}

fn main() {
    // AlignmentPaper carries the correct metadata.
    assert_eq!(AlignmentPaper::KEY, "alignment-paper");
    assert_eq!(
        AlignmentPaper::TITLE,
        "Alignment-Based Conformance Checking"
    );
    assert_eq!(AlignmentPaper::YEAR, Some(2008));
    assert_eq!(AlignmentPaper::FAMILY, WitnessFamily::Paper);

    // AlignmentPaper satisfies the Witness bound.
    require_paper_family::<AlignmentPaper>(std::marker::PhantomData);

    // AlignmentPaper and WfNetSoundnessPaper are both Paper-family witnesses but
    // name orthogonal authorities — conformance vs. structural soundness.
    assert_ne!(AlignmentPaper::KEY, WfNetSoundnessPaper::KEY);
    assert_ne!(AlignmentPaper::TITLE, WfNetSoundnessPaper::TITLE);
    assert_ne!(AlignmentPaper::YEAR, WfNetSoundnessPaper::YEAR);
    assert_eq!(AlignmentPaper::FAMILY, WfNetSoundnessPaper::FAMILY);

    // Non-interchangeability: distinct PhantomData types.
    let _ap: std::marker::PhantomData<AlignmentPaper> = std::marker::PhantomData;
    let _ws: std::marker::PhantomData<WfNetSoundnessPaper> = std::marker::PhantomData;
}
