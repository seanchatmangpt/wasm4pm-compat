//! Process Cube framework shapes — van der Aalst (2013).
//!
//! Demonstrates the `process_cube` module vocabulary:
//!
//! - [`CubeDimension<NAME>`] — const-param axis label (distinct types per name)
//! - [`CubeDimensionKind`] — six semantic dimension kinds with `Display`
//! - [`CubeSlice<D, V>`] — dimension + concrete value binding
//! - [`CubeCell<DIMS>`] — cell at the intersection of N dimension slices
//! - [`CubeProjectionWitness<FROM, TO>`] — receipt shape for a dimension reduction
//! - [`ProcessCube<Log, DIMS>`] — top-level cube shape declaration
//! - [`CellComparison<DIMS>`] — structural shape for a cross-cell comparison
//!
//! **The process cube is multi-dimensional process analysis:** you slice an
//! event log by resource, time, activity, or OCEL object type and compare the
//! sub-logs per cell. This module carries the *shapes* of those operations;
//! it does NOT compute sub-logs, discover models, or compare cells.
//! Graduate to `wasm4pm` for the computation.
//!
//! **Failure witness:** `dim_count()`, `from_dims()`, `to_dims()`,
//! `dimension_count()`, and `Display` strings are all asserted — renames or
//! removal break this example.
//!
//! Doc reference: `src/process_cube.rs`, `docs/API_TOUR.md`

use core::marker::PhantomData;
use wasm4pm_compat::process_cube::{
    CellComparison, CubeCell, CubeDimension, CubeDimensionKind, CubeProjectionWitness,
    CubeSlice, ProcessCube,
};

// Dummy log type used as the phantom parameter.
struct OrderManagementLog;

fn main() {
    println!("=== Process Cube shapes (van der Aalst 2013) ===\n");

    // ── Part 1: CubeDimension<NAME> — distinct types per name ────────────────
    println!("Part 1: CubeDimension<NAME> — const-param axis labels");

    let _resource_dim: CubeDimension<"resource"> = CubeDimension;
    let _time_dim:     CubeDimension<"time">     = CubeDimension;
    let _activity_dim: CubeDimension<"activity"> = CubeDimension;
    // CubeDimension<"resource"> and CubeDimension<"time"> are DIFFERENT types.
    // The compiler prevents substituting one for the other.
    println!("  ✓ CubeDimension<\"resource\">, <\"time\">, <\"activity\"> are distinct types");

    // ── Part 2: CubeDimensionKind — semantic axis kinds with Display ──────────
    println!("\nPart 2: CubeDimensionKind — six semantic kinds");

    let kinds = [
        (CubeDimensionKind::Activity,      "activity"),
        (CubeDimensionKind::Resource,      "resource"),
        (CubeDimensionKind::Time,          "time"),
        (CubeDimensionKind::DataAttribute, "data-attribute"),
        (CubeDimensionKind::ObjectType,    "object-type"),
        (CubeDimensionKind::CaseAttribute, "case-attribute"),
    ];
    for (kind, expected) in &kinds {
        let displayed = format!("{kind}");
        assert_eq!(&displayed, expected, "CubeDimensionKind::Display mismatch");
        println!("  ✓ {expected}");
    }

    // ── Part 3: CubeSlice<D, V> — dimension + value binding ──────────────────
    println!("\nPart 3: CubeSlice<D, V>");

    let resource_slice: CubeSlice<CubeDimension<"resource">, &str> = CubeSlice {
        dimension: PhantomData,
        value: "Alice",
    };
    assert_eq!(resource_slice.value, "Alice");
    println!("  ✓ resource slice: value = \"{}\"", resource_slice.value);

    let time_slice: CubeSlice<CubeDimension<"time">, u32> = CubeSlice {
        dimension: PhantomData,
        value: 2024,
    };
    assert_eq!(time_slice.value, 2024);
    println!("  ✓ time slice: value = {} (year)", time_slice.value);

    // ── Part 4: CubeCell<DIMS> — intersection of N dimension slices ──────────
    println!("\nPart 4: CubeCell<DIMS>");

    let cell_2d: CubeCell<2> = CubeCell::new();
    assert_eq!(cell_2d.dim_count(), 2);
    println!("  ✓ CubeCell::<2>::new().dim_count() = 2");

    let cell_3d: CubeCell<3> = CubeCell::default();
    assert_eq!(cell_3d.dim_count(), 3);
    println!("  ✓ CubeCell::<3>::default().dim_count() = 3");

    // CubeCell<2> and CubeCell<3> are distinct types — cell dimensionality is
    // enforced at compile time, not at runtime.
    println!("  ✓ CubeCell<2> and CubeCell<3> are distinct types");

    // ── Part 5: CubeProjectionWitness<FROM, TO> — projection receipt ─────────
    println!("\nPart 5: CubeProjectionWitness<FROM_DIMS, TO_DIMS>");

    let w: CubeProjectionWitness<4, 2> = CubeProjectionWitness::new();
    assert_eq!(w.from_dims(), 4);
    assert_eq!(w.to_dims(), 2);
    println!("  ✓ CubeProjectionWitness<4,2>: from=4, to=2");

    // The from > to invariant is structural (enforced by the caller's intent,
    // not checked at runtime here — the engine graduate validates it).
    let identity: CubeProjectionWitness<3, 3> = CubeProjectionWitness::default();
    assert_eq!(identity.from_dims(), identity.to_dims());
    println!("  ✓ CubeProjectionWitness<3,3>: identity projection (from=to)");

    // ── Part 6: ProcessCube<Log, DIMS> — top-level cube declaration ──────────
    println!("\nPart 6: ProcessCube<Log, DIMS>");

    // A 3-dimensional cube over an order-management log.
    let cube: ProcessCube<OrderManagementLog, 3> = ProcessCube::new();
    assert_eq!(cube.dimension_count(), 3);
    println!("  ✓ ProcessCube<OrderManagementLog, 3>.dimension_count() = 3");

    let cube_2d: ProcessCube<OrderManagementLog, 2> = ProcessCube::default();
    assert_eq!(cube_2d.dimension_count(), 2);
    println!("  ✓ ProcessCube::default() → dimension_count = 2");

    // ── Part 7: CellComparison<DIMS> — cross-cell comparison shape ───────────
    println!("\nPart 7: CellComparison<DIMS>");

    // A comparison between two 2-dimensional cells (e.g., Alice vs Bob in Q1).
    let cmp: CellComparison<2> = CellComparison {
        cell_a: CubeCell::new(),
        cell_b: CubeCell::new(),
    };
    assert_eq!(cmp.cell_a.dim_count(), 2);
    assert_eq!(cmp.cell_b.dim_count(), 2);
    println!("  ✓ CellComparison<2>: cell_a.dim_count=2, cell_b.dim_count=2");

    // ── Part 8: Realistic slice composition ───────────────────────────────────
    println!("\nPart 8: Realistic slice composition (Alice vs Bob in Q1 2024)");

    // A 3D cube: resource × time × activity.
    let _cube_3d: ProcessCube<OrderManagementLog, 3> = ProcessCube::new();

    // Alice's slice at (resource=Alice, time=2024-Q1).
    let alice_resource: CubeSlice<CubeDimension<"resource">, &str> = CubeSlice {
        dimension: PhantomData, value: "Alice",
    };
    let q1_time: CubeSlice<CubeDimension<"time">, &str> = CubeSlice {
        dimension: PhantomData, value: "2024-Q1",
    };
    // Bob's slice at (resource=Bob, time=2024-Q1).
    let bob_resource: CubeSlice<CubeDimension<"resource">, &str> = CubeSlice {
        dimension: PhantomData, value: "Bob",
    };

    assert_eq!(alice_resource.value, "Alice");
    assert_eq!(q1_time.value, "2024-Q1");
    assert_eq!(bob_resource.value, "Bob");
    assert_ne!(alice_resource.value, bob_resource.value);

    let comparison: CellComparison<2> = CellComparison {
        cell_a: CubeCell::new(), // Alice × Q1
        cell_b: CubeCell::new(), // Bob × Q1
    };
    assert_eq!(comparison.cell_a.dim_count(), 2);
    println!("  ✓ Alice×Q1 vs Bob×Q1 comparison: both cells at 2 dimensions");
    println!("  ✓ Graduate to wasm4pm to extract sub-logs, discover models per cell,");
    println!("    and compute fitness differences between Alice's and Bob's process.");

    println!("\n=== All assertions passed — process_cube module surface is witnessed ===");
    println!("  Covered: CubeDimension<N> (const-param axis), CubeDimensionKind (6 kinds),");
    println!("           CubeSlice, CubeCell, CubeProjectionWitness, ProcessCube,");
    println!("           CellComparison, realistic slice composition.");
    println!("  Graduate to wasm4pm for: sub-log extraction, cell discovery, cross-cell comparison.");
}
