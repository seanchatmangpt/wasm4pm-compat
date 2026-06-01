# Paper Classification Methodology

Every paper in `docs/PAPER_COVERAGE_LEDGER.md` receives one of five statuses.
This document defines what each status means and how to assign it.

## COVERED_BY_TYPE

The paper introduces formal objects (types, structures, laws) that are directly encoded
as zero-cost Rust types in this crate. The structural shapes are present; any execution
logic is absent.

**Test:** can you point to a `src/*.rs` file with types named after the paper's formal objects?

## COVERED_BY_GRADUATION_BOUNDARY

The paper's primary contribution is an *algorithm* (discovery, conformance checking, replay,
alignment, prediction, query evaluation). The algorithm's **output shapes** may be typed here,
but the algorithm itself graduates to `wasm4pm`. The ledger entry names what specifically
graduates using a `GraduationReason` variant.

**Test:** does `docs/GRADUATION_BOUNDARIES.md` name this paper's algorithm as graduating?

## PARTIAL_WITH_REASON

Some of the paper's formal objects are typed here; others are missing or incorrectly
classified. The ledger entry names what is absent and why.

**Test:** are there named types in `src/` for *some* but not *all* formal objects?

## DUPLICATE_OR_BACKGROUND

The paper repeats or summarises content from another ledgered paper, or provides
background context without introducing new formal objects.

**Test:** does another ledger row cover the same formal objects with a more primary citation?

## OUT_OF_SCOPE_WITH_REASON

The paper does not introduce process-mining formal objects. The reason must be specific
(e.g. "DevOps focus, no formal process model", "general AI survey, no event log structures").

**Test:** does the paper introduce any formal object that could be a zero-cost Rust type?
If yes, it cannot be OUT_OF_SCOPE — reconsider COVERED_BY_TYPE or PARTIAL.

## The forbidden status

`MISSING_TYPE_LAW` is not a valid status. A paper either has coverage, a named graduation
boundary, a documented partial, or a specific out-of-scope reason. Hidden gaps are defects.
