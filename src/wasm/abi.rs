//! WASM memory ABI audit helpers.
//! Enforces memory safety checks and alignment on the threshold.

/// Audits pointer alignment and containment of linear memory pointers for JS-to-Rust crossings.
///
/// Returns `true` if the pointer range `[ptr, ptr + len)` is validly aligned for
/// `align`, does not overflow the address space, and is within the boundaries
/// of the WASM linear memory space if run in a WASM environment.
pub fn verify_abi_memory_safety(ptr: usize, len: usize, align: usize) -> bool {
    if len == 0 {
        return true;
    }
    // Check alignment constraints
    if align == 0 || !align.is_power_of_two() {
        return false;
    }
    if !ptr.is_multiple_of(align) {
        return false;
    }
    // Check overflow
    let end = match ptr.checked_add(len) {
        Some(val) => val,
        None => return false,
    };

    #[cfg(target_arch = "wasm32")]
    {
        // memory_size(0) returns the size in pages (64 KiB each)
        let pages = core::arch::wasm32::memory_size(0);
        let max_bytes = pages * 65536;
        end <= max_bytes
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        // On non-wasm targets (e.g. native testing), we just check that the address space
        // check did not overflow (which we already did).
        let _ = end;
        true
    }
}

/// Verifies that two pointer ranges `[ptr1, ptr1 + len1)` and `[ptr2, ptr2 + len2)`
/// do not overlap and are both free from overflow.
pub fn verify_disjoint(ptr1: usize, len1: usize, ptr2: usize, len2: usize) -> bool {
    if len1 == 0 || len2 == 0 {
        return true;
    }
    let end1 = match ptr1.checked_add(len1) {
        Some(val) => val,
        None => return false,
    };
    let end2 = match ptr2.checked_add(len2) {
        Some(val) => val,
        None => return false,
    };

    end1 <= ptr2 || end2 <= ptr1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_abi_memory_safety_alignment() {
        // Valid aligned pointer
        assert!(verify_abi_memory_safety(1024, 64, 8));
        assert!(verify_abi_memory_safety(1024, 64, 1));

        // Misaligned pointer
        assert!(!verify_abi_memory_safety(1023, 64, 8));
        assert!(!verify_abi_memory_safety(1021, 64, 4));

        // Invalid alignment parameter (not power of two)
        assert!(!verify_abi_memory_safety(1024, 64, 7));
        assert!(!verify_abi_memory_safety(1024, 64, 0));
    }

    #[test]
    fn test_verify_abi_memory_safety_overflow() {
        // Safe range
        assert!(verify_abi_memory_safety(100, 100, 4));

        // Overflowing range
        assert!(!verify_abi_memory_safety(usize::MAX - 10, 20, 1));
    }

    #[test]
    fn test_verify_disjoint() {
        // Disjoint ranges
        assert!(verify_disjoint(100, 50, 150, 50));
        assert!(verify_disjoint(150, 50, 100, 50));

        // Overlapping ranges
        assert!(!verify_disjoint(100, 51, 150, 50));
        assert!(!verify_disjoint(149, 50, 100, 50));

        // Adjoining but not overlapping
        assert!(verify_disjoint(100, 50, 150, 10));
        assert!(verify_disjoint(150, 10, 100, 50));

        // Zero-length cases
        assert!(verify_disjoint(100, 0, 100, 50));
        assert!(verify_disjoint(100, 50, 120, 0));

        // Overflowing range checks
        assert!(!verify_disjoint(usize::MAX - 10, 20, 100, 50));
    }
}
