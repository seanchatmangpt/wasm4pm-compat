// COMPILE-PASS: IsEmpty trait — proves IsEmpty blanket impls work for Vec<T>, &[T], and &str

use wasm4pm_compat::loss::IsEmpty;

fn check_empty<T: IsEmpty>(val: T) -> bool {
    val.is_empty()
}

fn main() {
    // Vec<T>
    assert!(check_empty(Vec::<u8>::new()));
    assert!(!check_empty(vec![1u8]));

    // &[T]
    let empty_slice: &[u8] = &[];
    let non_empty_slice: &[u8] = &[1];
    assert!(check_empty(empty_slice));
    assert!(!check_empty(non_empty_slice));

    // &str
    assert!(check_empty(""));
    assert!(!check_empty("hello"));
}
