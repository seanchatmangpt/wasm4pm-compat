import fs from 'fs';
import { 
    EventLogSchema, 
    OcelLogSchema, 
    PetriNetSchema, 
    DFGSchema, 
    DeclareModelSchema,
    TokenReplayResultSchema,
    ConformanceResultSchema,
    ReceiptSchema
} from './zod_schemas';

/**
 * Comprehensive demo covering all major process mining shapes using real wasm4pm data.
 */

function runComprehensiveDemo() {
    console.log("🌟 wasm4pm-compat Comprehensive Zod Validation Demo (Real Data) 🌟\n");

    // 1. Petri Net (from wf-net.json)
    const net = JSON.parse(fs.readFileSync('./data/petri_net.json', 'utf8'));
    PetriNetSchema.parse(net);
    console.log("✅ Petri Net Schema Validated (wf-net.json)");

    // 2. DFG
    const dfg = {
        nodes: [{ activity: "A", frequency: 10 }, { activity: "B", frequency: 8 }],
        edges: [{ source: "A", target: "B", frequency: 5 }]
    };
    DFGSchema.parse(dfg);
    console.log("✅ DFG Schema Validated");

    // 3. Declare Model
    const declare = {
        constraints: [
            { constraint_type: "Response", activities: ["A", "B"] }
        ]
    };
    DeclareModelSchema.parse(declare);
    console.log("✅ Declare Model Schema Validated");

    // 4. OCEL Log (from ocel.json)
    const ocel = JSON.parse(fs.readFileSync('./data/ocel.json', 'utf8'));
    OcelLogSchema.parse(ocel);
    console.log("✅ OCEL Log Schema Validated (ocel-v2.json)");

    // 5. Conformance Results
    const replay = {
        fitness: 0.85,
        produced_tokens: 100,
        consumed_tokens: 80,
        missing_tokens: 5,
        remaining_tokens: 15
    };
    TokenReplayResultSchema.parse(replay);
    console.log("✅ Token Replay Result Validated");

    // 6. Cryptographic Receipt
    const receipt = {
        model_id: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
        final_hash_chain: "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef12345678",
        state: "Admitted",
        witness: "Ocel20",
        verdict: { is_perfect: false }
    };
    // Receipt actually uses EvidenceSchema or nested fields
    // For this demo we use the ReceiptSchema directly if it matches
    // Based on our ontology: Receipt has model_id, final_hash_chain, verdict
    ReceiptSchema.parse(receipt);
    console.log("✅ Receipt Schema Validated");

    console.log("\n🚀 All structural laws enforced successfully via Zod!");
}

try {
    runComprehensiveDemo();
} catch (e) {
    console.error("❌ Validation Failed:", e);
    process.exit(1);
}
