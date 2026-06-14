//! Nightly foundry — four paper-derived type-law surfaces (`src/nightly_foundry.rs`).
//!
//! This example exercises all 5 pub items in the `nightly_foundry` module:
//!   - `petri_law` mod: Marking, PreMatrix, PostMatrix (Murata 1989)
//!   - `powl_law` mod: TypedNode<KIND> const-kind-tagged POWL fragments (Kourani arXiv:2505.07052)
//!   - `evidence_law` mod: EvidenceKind specialization — "raw" vs "admitted" at compile time
//!   - `token_law` mod: SIMD token-enabling check (Murata §2 enabling condition)
//!   - `families_match_simd` fn: SIMD witness-family batch check
//!
//! Nightly features required:
//!   generic_const_exprs — Petri arc matrices [u8; P * T]
//!   adt_const_params    — PowlKind as const param in TypedNode<KIND>
//!   min_specialization  — EvidenceKind "raw" vs "admitted" blanket + specialized impl
//!   portable_simd       — SIMD enabling/firing checks

#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
#![feature(const_trait_impl)]
#![feature(min_specialization)]
#![feature(portable_simd)]
#![allow(incomplete_features)]

use wasm4pm_compat::nightly_foundry::{
    evidence_law::{Admitted, EvidenceKind},
    families_match_simd,
    petri_law::{Marking, PostMatrix, PreMatrix},
    powl_law::{OrderEdge, TypedNode},
    token_law,
};
use wasm4pm_compat::witness::WitnessFamily;

fn main() {
    // ── petri_law: Marking, PreMatrix, PostMatrix ─────────────────────────────
    println!("== petri_law: Murata (1989) §2 arc matrices ==");

    // Marking: const EMPTY and total_tokens
    const M0: Marking<3> = Marking::EMPTY;
    assert_eq!(M0.total_tokens(), 0, "EMPTY marking has 0 tokens");
    let m1 = Marking([1u32, 2u32, 0u32]);
    assert_eq!(m1.total_tokens(), 3, "total_tokens = sum");
    assert_eq!(m1.at(1), Some(2), "at(1) = 2");
    assert_eq!(m1.at(5), None, "at(out_of_bounds) = None");
    println!("  Marking<3>::EMPTY.total_tokens() = {}", M0.total_tokens());
    println!("  Marking([1,2,0]).total_tokens() = {}", m1.total_tokens());

    // PreMatrix + PostMatrix: enabling and firing
    // 2 places, 1 transition. p0 → t0 → p1.
    let mut pre = PreMatrix::<2, 1>::ZERO;
    pre.weights[0] = 1; // W⁻(p0, t0) = 1
    let mut post = PostMatrix::<2, 1>::ZERO;
    post.weights[1] = 1; // W⁺(t0, p1) = 1

    let m_before = Marking([1u32, 0u32]);
    assert!(pre.is_enabled(0, &m_before), "t0 enabled");
    let m_after = post.fire(0, m_before, &pre);
    assert_eq!(m_after, Marking([0u32, 1u32]), "token moved p0 → p1");
    println!("  PreMatrix: t0 enabled with M=[1,0]: {}", pre.is_enabled(0, &m_before));
    println!("  PostMatrix: fire t0 -> M'=[0,1]: {:?}", m_after.0);

    // Marking with 0 tokens blocks transition
    let m_empty = Marking([0u32, 0u32]);
    assert!(!pre.is_enabled(0, &m_empty), "t0 blocked on empty marking");
    println!("  t0 blocked with M=[0,0]: {}", !pre.is_enabled(0, &m_empty));

    // ── powl_law: TypedNode<KIND> ─────────────────────────────────────────────
    println!("\n== powl_law: Kourani (arXiv:2505.07052) POWL fragment kinds ==");

    // Atom: observable
    let atom = TypedNode::atom(1u32);
    assert!(atom.is_observable(), "Atom is observable");
    assert_eq!(atom.id(), 1u32, "Atom id");

    // Silent: not observable
    let silent = TypedNode::silent(2u32);
    assert!(!silent.is_observable(), "Silent is not observable");
    assert_eq!(silent.id(), 2u32);

    // Partial: concurrency check via OrderEdge
    let partial = TypedNode::partial(0u32);
    let edges = [OrderEdge { before: 1, after: 2 }];
    assert!(!partial.are_concurrent(&edges, 1, 2), "1 ≺ 2: not concurrent");
    assert!(partial.are_concurrent(&edges, 1, 3), "no edge 1↔3: concurrent");
    assert!(partial.are_concurrent(&[], 5, 6), "no edges: all concurrent");

    // Xor: min_branches
    let xor = TypedNode::xor(3u32);
    assert_eq!(TypedNode::<{ wasm4pm_compat::nightly_foundry::powl_law::PowlKind::Xor }>::min_branches(), 2);
    assert_eq!(xor.id(), 3u32);

    // Loop node
    let lp = TypedNode::loop_node(4u32);
    assert_eq!(lp.id(), 4u32);

    println!("  Atom(1).is_observable()  = {}", atom.is_observable());
    println!("  Silent(2).is_observable() = {}", silent.is_observable());
    println!("  Partial: 1≺2 concurrent? {} | 1↔3 concurrent? {}",
        partial.are_concurrent(&edges, 1, 2),
        partial.are_concurrent(&edges, 1, 3));
    println!("  Xor min_branches = {}", TypedNode::<{ wasm4pm_compat::nightly_foundry::powl_law::PowlKind::Xor }>::min_branches());
    println!("  Loop id = {}", lp.id());

    // ── evidence_law: EvidenceKind specialization ─────────────────────────────
    println!("\n== evidence_law: EvidenceKind blanket + specialized impl ==");

    // Any T that is not Admitted<_> → "raw"
    let raw_val: u64 = 42u64;
    assert_eq!(raw_val.kind_label(), "raw", "u64 is raw");
    assert_eq!("hello".kind_label(), "raw", "&str is raw");

    // Admitted<T> → "admitted"
    let admitted = Admitted(42u64);
    assert_eq!(admitted.kind_label(), "admitted", "Admitted<u64> is admitted");
    let admitted_str = Admitted("world");
    assert_eq!(admitted_str.kind_label(), "admitted", "Admitted<&str> is admitted");

    // Zero-cost: Admitted is repr(transparent) — same size as T
    assert_eq!(std::mem::size_of::<Admitted<u64>>(), std::mem::size_of::<u64>());

    println!("  u64::kind_label()          = \"{}\"", raw_val.kind_label());
    println!("  Admitted<u64>::kind_label() = \"{}\"", admitted.kind_label());
    println!("  Admitted<u64> size == u64 size: {} bytes", std::mem::size_of::<u64>());

    // ── token_law: SIMD enabling check ────────────────────────────────────────
    println!("\n== token_law: SIMD Murata §2 enabling condition ==");

    // transition_enabled_4: all places have enough tokens
    assert!(token_law::transition_enabled_4([2, 1, 3, 0], [1, 1, 2, 0]), "enabled: all ≥ pre");
    assert!(!token_law::transition_enabled_4([2, 0, 3, 0], [1, 1, 2, 0]), "blocked: place[1]=0 < 1");

    // fire_4: M' = M - pre + post
    let m_fired = token_law::fire_4([2, 1, 3, 0], [1, 1, 2, 0], [0, 0, 0, 1]);
    assert_eq!(m_fired, [1, 0, 1, 1], "M' = M - pre + post");

    // transition_enabled_8
    assert!(token_law::transition_enabled_8([1,1,1,1,1,1,1,1], [1,1,1,1,1,1,1,1]), "8-place enabled");
    assert!(!token_law::transition_enabled_8([1,1,0,1,1,1,1,1], [1,1,1,1,1,1,1,1]), "8-place blocked");

    println!("  transition_enabled_4([2,1,3,0] ≥ [1,1,2,0]): {}", token_law::transition_enabled_4([2,1,3,0],[1,1,2,0]));
    println!("  transition_enabled_4([2,0,3,0] ≥ [1,1,2,0]): {}", token_law::transition_enabled_4([2,0,3,0],[1,1,2,0]));
    println!("  fire_4([2,1,3,0] - [1,1,2,0] + [0,0,0,1]): {:?}", m_fired);
    println!("  transition_enabled_8 all-1 ≥ all-1: {}", token_law::transition_enabled_8([1;8],[1;8]));

    // ── families_match_simd: batch witness family check ───────────────────────
    println!("\n== families_match_simd: SIMD witness family batch check ==");

    let all_paper = [WitnessFamily::Paper; 8];
    let mask = families_match_simd(all_paper, WitnessFamily::Paper);
    assert_eq!(mask, 0b1111_1111u8, "all 8 Paper → all bits set");
    println!("  all Paper mask = 0b{:08b} (0xFF)", mask);

    // bits 1 and 4 are Standard — should be unset
    let mixed = [
        WitnessFamily::Paper, WitnessFamily::Standard,
        WitnessFamily::Paper, WitnessFamily::Paper,
        WitnessFamily::Standard, WitnessFamily::Paper,
        WitnessFamily::Paper, WitnessFamily::Paper,
    ];
    let mask2 = families_match_simd(mixed, WitnessFamily::Paper);
    assert_eq!(mask2, 0b1110_1101u8, "mixed: bits 1 and 4 unset");
    println!("  mixed mask     = 0b{:08b}", mask2);

    println!("\nEXIT 0");
}
