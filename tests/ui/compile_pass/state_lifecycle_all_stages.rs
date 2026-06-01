// Law: StateLifecycleAllStagesLaw — all seven lifecycle stage tokens (Raw, Parsed, Admitted, Refused, Projected, Exportable, Receipted) are zero-sized uninhabited enums that compile as PhantomData tags
// COMPILE-PASS: All state lifecycle stage tokens — proves all seven lifecycle stage enums compile

use core::marker::PhantomData;
use wasm4pm_compat::state::{
    Admitted, Exportable, Parsed, Projected, Raw, Receipted, Refused,
};

fn main() {
    // All stages are uninhabited enums used as PhantomData markers.
    let _ = PhantomData::<Raw>;
    let _ = PhantomData::<Parsed>;
    let _ = PhantomData::<Admitted>;
    let _ = PhantomData::<Refused>;
    let _ = PhantomData::<Projected>;
    let _ = PhantomData::<Exportable>;
    let _ = PhantomData::<Receipted>;
}
