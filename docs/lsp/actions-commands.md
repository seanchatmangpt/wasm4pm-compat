# LSP Code Actions and Custom Commands

This document specifies the Language Server Protocol (LSP) Code Actions and Custom Commands designed for the `wasm4pm-compat` process evidence ecosystem. These mechanisms support developers in maintaining structural integrity, keeping boundaries honest, and satisfying the named laws of well-formed compatibility surfaces.

---

## 1. LSP Code Actions

LSP Code Actions allow editors to trigger refactorings and quick-fixes when compatibility laws are violated. The server supports three primary code actions.

### 1.1 Generating an `Admit` Stub
*   **Triggering Diagnostic:** `RawEvidenceExportedAsAdmitted` (W4PM-003)
*   **LSP Code Action Kind:** `quickfix` / `refactor.rewrite`
*   **Purpose:** Automatically creates a trait implementation structure to route untrusted raw data into admitted evidence, ensuring that raw values do not bypass boundary validation.
*   **Code Generation Template:**
    ```rust
    impl wasm4pm_compat::admission::Admit for [TypeName] {
        type Raw = [RawType];
        type Admitted = [AdmittedType];
        type Reason = [ReasonType];
        type Witness = [WitnessType];

        fn admit(
            raw: wasm4pm_compat::evidence::Evidence<Self::Raw, wasm4pm_compat::state::Raw, Self::Witness>,
        ) -> Result<
            wasm4pm_compat::admission::Admission<Self::Admitted, Self::Witness>,
            wasm4pm_compat::admission::Refusal<Self::Reason, Self::Witness>,
        > {
            // Boundary validation: check if raw evidence is valid under this boundary's law.
            let is_valid = true;
            if is_valid {
                Ok(wasm4pm_compat::admission::Admission::new(raw.value.into()))
            } else {
                Err(wasm4pm_compat::admission::Refusal::new([DefaultReasonVariant]))
            }
        }
    }
    ```

### 1.2 Generating a `LossReport` Stub
*   **Triggering Diagnostic:** `HiddenFlattening` or `LossyProjectionWithoutPolicy` (W4PM-002)
*   **LSP Code Action Kind:** `quickfix` / `refactor.rewrite`
*   **Purpose:** Builds a standard, structured receipt of lossy translations (e.g. object-centric logs down to case-centric logs), preventing silent data loss and making boundary limitations auditable.
*   **Code Generation Template:**
    ```rust
    wasm4pm_compat::loss::LossReport::<[FromType], [ToType], [ItemType]>::new(
        wasm4pm_compat::loss::ProjectionName("[ProjectionName]"),
        wasm4pm_compat::loss::LossPolicy::AllowLossWithReport,
        [LostItemsExpression],
    )
    ```

### 1.3 Swapping Panic/Unwrap with Typed Refusal Returns
*   **Triggering Diagnostic:** `MissingRefusalPath` (W4PM-ADM-001)
*   **LSP Code Action Kind:** `quickfix`
*   **Purpose:** Replaces runtime panics and unsafe unwraps inside `Admit` or `Project` blocks with structured, typed refusal responses. Refusals are treated as first-class boundary verdicts rather than execution errors.
*   **Code Transformation Logic:**
    1.  **Macro Replacement:** Replaces `panic!(...)` statements on the target line with:
        ```rust
        return Err(wasm4pm_compat::admission::Refusal::new([ReasonVariable]))
        ```
    2.  **Unwrap Replacement:** Replaces `.unwrap()` method calls on the target line with:
        ```rust
        .ok_or(wasm4pm_compat::admission::Refusal::new([ReasonVariable]))?
        ```

---

## 2. Custom Command: `w4pm.explainWhy`

To keep the compatibility boundary clear and help developers understand structural restrictions, the LSP server exposes a custom execution command: `w4pm.explainWhy`.

*   **Command Identifier:** `w4pm.explainWhy`
*   **Argument Structure:** `[CompatDiagnosticCode: string]`
*   **Response / Effect:** Retrieves a detailed markdown text explanation explaining the corresponding law, the rationale for its existence, and the remedy code to resolve it.

### 2.1 Detailed Explanation Catalog

The command maps standard diagnostic variant codes to detailed explanations:

#### 1. `MissingWitness`
*   **Law:** Every admitted/projected surface must name its authority (a `Witness` type).
*   **Rationale:** Process mining evidence has no semantics unless judged against a specific standard or paper (e.g., OCEL 2.0). Naming the witness prevents semantic drift and establishes a clear trust chain.
*   **Remedy:** Specify a witness type (e.g. `Ocel20`, `WfNetSoundnessPaper`) in your evidence signatures or traits.
*   **Code Example:**
    ```rust
    use wasm4pm_compat::evidence::Evidence;
    use wasm4pm_compat::witness::Ocel20;

    let admitted: Evidence<MyShape, Admitted, Ocel20> = ...
    ```

#### 2. `MissingRoundTripFixture`
*   **Law:** A round-trip claim (import then export) must be backed by a test fixture.
*   **Rationale:** Round-trip claims are compatibility promises. If you claim that your format can be imported and subsequently exported back, you must have an actual test case to verify that no structural regression occurs.
*   **Remedy:** Implement a test that imports and then exports a sample structure, asserting equivalence.
*   **Code Example:**
    ```rust
    #[test]
    fn test_round_trip() {
        let original = get_test_fixture_data();
        let imported = MyShape::import(original.clone()).unwrap();
        let exported = imported.export().unwrap();
        assert_eq!(original, exported);
    }
    ```

#### 3. `RawEvidenceExportedAsAdmitted`
*   **Law:** `Raw` (untrusted) evidence may not leave the compatibility boundary as if it were `Admitted`.
*   **Rationale:** Raw values are unvetted. Allowing them to bypass `Admit` trait verification breaks safety and lifecycle guarantees downstream.
*   **Remedy:** Route the raw evidence through an `Admit` implementation to generate a valid `Admission`.
*   **Code Example:**
    ```rust
    use wasm4pm_compat::admission::Admit;

    let raw_evidence = Evidence::raw(data);
    let admitted = MyAdmitImpl::admit(raw_evidence)?;
    ```

#### 4. `LossyProjectionWithoutPolicy`
*   **Law:** Any lossy projection must be governed by an explicit `LossPolicy`.
*   **Rationale:** Discarding structural details silently makes auditing impossible. The caller must explicitly state the policy under which data loss is tolerated (refuse, allow named, or allow with report).
*   **Remedy:** Implement the projection under the `Project` trait using an explicit policy.
*   **Code Example:**
    ```rust
    impl wasm4pm_compat::loss::Project for MyProjection {
        type Policy = LossPolicy::AllowLossWithReport;
    }
    ```

#### 5. `HiddenFlattening`
*   **Law:** Structure must not be discarded silently (no secret flattening).
*   **Rationale:** Silent data loss compromises evidence integrity. The boundary must produce an itemized `LossReport` so downstream reviewers are aware of what was pruned.
*   **Remedy:** Emit a `LossReport` alongside the projected result.
*   **Code Example:**
    ```rust
    use wasm4pm_compat::loss::{LossPolicy, LossReport, ProjectionName};

    let report = LossReport::<FromType, ToType, Vec<String>>::new(
        ProjectionName("ocel-flatten"),
        LossPolicy::AllowLossWithReport,
        vec!["dropped_object_links".to_string()],
    );
    ```

#### 6. `MissingRefusalPath`
*   **Law:** Every admission/projection surface must offer a refusal path with a specific named reason.
*   **Rationale:** Compatibility boundaries must not fail with generic errors. The refusal must name the specific law variant that was broken (e.g. `EmptyEventLog`) so editors and client tools can report precise errors.
*   **Remedy:** Define a custom enum for your refusal reasons and return it inside `Refusal::new()`.
*   **Code Example:**
    ```rust
    #[derive(Debug, Clone, Copy)]
    pub enum RefusalReason {
        EmptyEventLog,
        MissingStartMarking,
    }
    ```

#### 7. `MissingReceiptShape`
*   **Law:** Provenance-bearing evidence must carry a receipt shape (`Receipted`).
*   **Rationale:** Receipts prove that evidence has been processed by a witness authority. Stripping this envelope breaks the chain of custody.
*   **Remedy:** Wrap the admitted evidence in a `Receipted` struct together with its witness.
*   **Code Example:**
    ```rust
    use wasm4pm_compat::state::Receipted;
    let receipted = Receipted::new(admitted, witness);
    ```

#### 8. `UnreachablePrimitive`
*   **Law:** Every shape declared in the compatibility catalog must be connected to an admission, projection, or export contract.
*   **Rationale:** Unconnected primitives represent unvetted shapes and dead code, cluttering the boundary definition.
*   **Remedy:** Wire the primitive to an `Admit`/`Project` contract or remove it from the catalog.

#### 9. `MigrationRecommended`
*   **Law:** When a boundary outgrows structure-only rules, it should graduate to execution semantics.
*   **Rationale:** Compatibility boundaries merely verify shapes. If you need to perform simulation, token replay, or run workflows, you must graduate to the execution engine.
*   **Remedy:** Port your trait definitions and contracts to the full `wasm4pm` engine.
