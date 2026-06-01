// Law: ShapeMarkersZeroSizedLaw — OcelShape, XesShape, and OcedShape are uninhabited zero-sized enum markers; they prevent cross-format confusion at PhantomData boundaries
// COMPILE-PASS: OcelShape/XesShape/OcedShape zero-sized markers — proves shape markers are uninhabited enums

use wasm4pm_compat::interop::{OcedShape, OcelShape, XesShape};

fn accepts_ocel_xes(_: core::marker::PhantomData<(OcelShape, XesShape)>) {}
fn accepts_ocel_oced(_: core::marker::PhantomData<(OcelShape, OcedShape)>) {}

fn main() {
    accepts_ocel_xes(core::marker::PhantomData);
    accepts_ocel_oced(core::marker::PhantomData);
}
