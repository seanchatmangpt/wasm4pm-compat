// COMPILE-FAIL: OCEL attribute law — OcelAttribute::integer() value cannot be passed
// where a float-specific function expects f64.
// Law: OcelAttributeValue::Integer(i64) and OcelAttributeValue::Float(f64) are distinct enum
// variants. Passing an integer attribute value where f64 is required is a type error.
use wasm4pm_compat::ocel::OcelAttributeValue;

fn requires_float(_v: f64) {}

fn main() {
    // OcelAttributeValue::Integer(42) is an i64 wrapped in an enum, not f64.
    // Attempting to use the inner i64 as f64 is a mismatched types error.
    let val = OcelAttributeValue::Integer(42i64);
    // This must fail: OcelAttributeValue::Integer is not f64.
    requires_float(val);
}
