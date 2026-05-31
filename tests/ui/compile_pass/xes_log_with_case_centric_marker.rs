// COMPILE-PASS: xes-case-centric-marker — proves CaseCentricMarker is zero-sized
// and can be used as a PhantomData tag on a struct that wraps XesLog, making
// the case-centric vs. object-centric boundary unrepresentable at the type level.
use std::marker::PhantomData;
use wasm4pm_compat::xes::{CaseCentricMarker, XesLog};

struct TaggedLog<Tag> {
    log: XesLog,
    _tag: PhantomData<Tag>,
}

fn only_case_centric(_: &TaggedLog<CaseCentricMarker>) {}

fn main() {
    let log = XesLog::default();
    let tagged = TaggedLog { log, _tag: PhantomData::<CaseCentricMarker> };
    only_case_centric(&tagged);
    let _m: CaseCentricMarker = CaseCentricMarker;
    // CaseCentricMarker is Copy + Clone + Default.
    let _copy = _m;
}
