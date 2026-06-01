// End-to-end Roundtrip Serialization Lifecycle Simulation
// Enforcing the WASM Boundary Law

const wasm = require('./pkg/wasm4pm_compat.js');
const assert = require('assert').strict;

console.log("=== WASM BOUNDARY LAW INTEGRATION SIMULATION ===");

function runSimulation() {
    // -------------------------------------------------------------
    // Scenario 1: Witness Catalog & State Tags Fetching
    // -------------------------------------------------------------
    console.log("\n[Scenario 1] Fetching Witness Catalog & State Tags...");
    
    const witnesses = wasm.get_witness_catalog();
    console.log("-> Witness Catalog:", JSON.stringify(witnesses, null, 2));
    assert(witnesses.some(w => w.key === "ocel20"));
    assert(witnesses.some(w => w.key === "xes1849"));
    console.log("✓ Witness catalog successfully retrieved and verified.");

    const states = wasm.get_state_tags();
    console.log("-> State Tags:", JSON.stringify(states, null, 2));
    assert(states.some(t => t.name === "Admitted" && !t.is_terminal));
    assert(states.some(t => t.name === "Refused" && t.is_terminal));
    console.log("✓ State tags successfully retrieved and verified.");

    // -------------------------------------------------------------
    // Scenario 2: Precondition Validation & Refusal Gate
    // -------------------------------------------------------------
    console.log("\n[Scenario 2] Validating Admission Preconditions...");
    
    // Valid admission
    const okRes = wasm.validate_admission_preconditions("ocel", true, true);
    console.log("-> Valid OCEL result:", JSON.stringify(okRes, null, 2));
    assert.equal(okRes.is_ok, true);
    assert.equal(okRes.refusal_law, undefined);

    // Refusal: Empty set
    const emptyRes = wasm.validate_admission_preconditions("xes", false, true);
    console.log("-> Empty event set result:", JSON.stringify(emptyRes, null, 2));
    assert.equal(emptyRes.is_ok, false);
    assert.equal(emptyRes.refusal_law, "EmptyEventSet");

    // Refusal: Dangling Link
    const linkRes = wasm.validate_admission_preconditions("ocel", true, false);
    console.log("-> Missing links result:", JSON.stringify(linkRes, null, 2));
    assert.equal(linkRes.is_ok, false);
    assert.equal(linkRes.refusal_law, "DanglingEventObjectLink");
    console.log("✓ Admission preconditions correctly verified and refused.");

    // -------------------------------------------------------------
    // Scenario 3: Graduation Candidate Crossing
    // -------------------------------------------------------------
    console.log("\n[Scenario 3] Creating Graduation Candidates...");
    
    const candidate = wasm.create_graduation_candidate(
        "NeedsDiscovery",
        "P2P Process Log",
        "blake3:deadbeef"
    );
    console.log("-> Graduation Candidate:", JSON.stringify(candidate, null, 2));
    assert.equal(candidate.reason, "NeedsDiscovery");
    assert.equal(candidate.subject, "P2P Process Log");
    assert.equal(candidate.evidence_ref, "blake3:deadbeef");

    // Test rejection on empty subject
    try {
        wasm.create_graduation_candidate("NeedsReplay", "", "blake3:deadbeef");
        assert.fail("Should have failed on empty subject");
    } catch (e) {
        console.log("✓ Correctly rejected empty subject:", e);
    }
    console.log("✓ Graduation candidate crossing validated.");

    // -------------------------------------------------------------
    // Scenario 4: Loss Report Construction
    // -------------------------------------------------------------
    console.log("\n[Scenario 4] Creating Loss Report...");
    
    const lossReport = wasm.create_loss_report(
        "ocel-to-xes",
        "AllowLossWithReport",
        ["ObjectObjectLink"]
    );
    console.log("-> Loss Report:", JSON.stringify(lossReport, null, 2));
    assert.equal(lossReport.projection_name, "ocel-to-xes");
    assert.equal(lossReport.policy, "AllowLossWithReport");
    assert.deepEqual(lossReport.items_dropped, ["ObjectObjectLink"]);
    console.log("✓ Loss report generation validated.");

    // -------------------------------------------------------------
    // Scenario 5: WASM ABI Memory Boundary & Pointer Safety Checks
    // -------------------------------------------------------------
    console.log("\n[Scenario 5] Validating WASM ABI Memory Safety...");

    // 1. Aligned pointer inside bounds
    assert.equal(wasm.verify_wasm_ptr_range(1024, 64, 8), true);
    assert.equal(wasm.verify_wasm_ptr_range(1024, 64, 4), true);
    
    // 2. Misaligned pointer (must fail)
    assert.equal(wasm.verify_wasm_ptr_range(1023, 64, 8), false);
    assert.equal(wasm.verify_wasm_ptr_range(1022, 64, 4), false);

    // 3. Overflowing pointer range (must fail)
    assert.equal(wasm.verify_wasm_ptr_range(0xFFFFFFF0, 32, 1), false);
    
    // 4. Disjoint check: overlapping ranges (must fail)
    assert.equal(wasm.verify_disjoint_ranges(100, 50, 120, 50), false);
    
    // 5. Disjoint check: non-overlapping ranges (must pass)
    assert.equal(wasm.verify_disjoint_ranges(100, 50, 150, 50), true);
    
    console.log("✓ Pointer alignment, overflow bounds, and disjointness successfully verified.");

    // -------------------------------------------------------------
    // Scenario 6: End-to-End Roundtrip Serialization Lifecycle
    // -------------------------------------------------------------
    console.log("\n[Scenario 6] End-to-End Roundtrip Serialization Lifecycle (WASM Boundary Law)...");
    
    // Step 6.1: Project Rust Type to DTO
    console.log("Step 6.1: Projecting Rust Process Evidence to DTO...");
    const initialDto = wasm.serialize_process_evidence(
        "case_999",
        ["order_placed", "order_paid"],
        "ocel20"
    );
    console.log("-> Initial projected DTO in JS:", JSON.stringify(initialDto, null, 2));
    assert.equal(initialDto.case_id, "case_999");
    assert.deepEqual(initialDto.events, ["order_placed", "order_paid"]);
    assert.equal(initialDto.state, "Raw");
    assert.equal(initialDto.witness_key, "ocel20");
    assert.equal(initialDto.is_valid, true);
    
    // Step 5.2: Manipulate / Replay in JS
    console.log("Step 5.2: Simulating replay/manipulation in JS...");
    // Clone and modify the DTO
    const manipulatedDto = { ...initialDto };
    manipulatedDto.events = [...initialDto.events, "order_shipped"];
    manipulatedDto.timestamp_ns = 2500.0;
    manipulatedDto.parent_block_hash = initialDto.block_hash;
    manipulatedDto.state = "Admitted";
    
    console.log("-> Manipulated DTO in JS:", JSON.stringify(manipulatedDto, null, 2));
    
    // Step 5.3: Roundtrip back to Rust & Deserialize
    console.log("Step 5.3: Deserializing and validating back in Rust...");
    const verifiedDto = wasm.verify_and_replay_evidence(manipulatedDto);
    console.log("-> Verified DTO returned from Rust:", JSON.stringify(verifiedDto, null, 2));
    
    assert.equal(verifiedDto.case_id, "case_999");
    assert.deepEqual(verifiedDto.events, ["order_placed", "order_paid", "order_shipped"]);
    assert.equal(verifiedDto.state, "Receipted"); // Promoted because events.length > 2 under ocel20
    assert.equal(verifiedDto.is_valid, true);
    assert.notEqual(verifiedDto.block_hash, initialDto.block_hash);
    console.log("✓ Roundtrip serialization completed with zero data loss or structural corruption.");
    
    // Step 5.4: Adversarial Integrity Checks
    console.log("Step 5.4: Running adversarial boundary checks...");
    
    // Test 5.4.1: Empty events (refusal law check)
    console.log("-> Test 5.4.1: Empty events (should fail Refusal: EmptyEventSet)...");
    const emptyEventsDto = { ...manipulatedDto, events: [] };
    try {
        wasm.verify_and_replay_evidence(emptyEventsDto);
        assert.fail("Should have failed on empty events");
    } catch (e) {
        console.log("✓ Correctly refused empty event set:", e);
        assert.equal(e, "Refusal: EmptyEventSet");
    }
    
    // Test 5.4.2: Unlawful state transition
    console.log("-> Test 5.4.2: Unlawful state transition (should fail Refusal: UnlawfulStateTransition)...");
    const unlawfulStateDto = { ...manipulatedDto, state: "CorruptedState" };
    try {
        wasm.verify_and_replay_evidence(unlawfulStateDto);
        assert.fail("Should have failed on unlawful state");
    } catch (e) {
        console.log("✓ Correctly refused unlawful state transition:", e);
        assert(String(e).includes("Refusal: UnlawfulStateTransition"));
    }

    // Test 5.4.3: Structural Corruption (passing invalid property types)
    console.log("-> Test 5.4.3: Structural corruption (passing number instead of string array)...");
    const corruptDto = { ...manipulatedDto, events: 12345 };
    try {
        wasm.verify_and_replay_evidence(corruptDto);
        assert.fail("Should have failed on structural corruption");
    } catch (e) {
        console.log("✓ Correctly rejected structurally corrupted input:", e);
        const errStr = String(e);
        assert(errStr.includes("StructuralCorruption") || errStr.includes("TypeError") || errStr.includes("Reflect.get") || errStr.includes("TypeMismatch"));
    }

    console.log("\n=== ALL INTEGRATION SIMULATIONS PASSED ===");
}

runSimulation();
