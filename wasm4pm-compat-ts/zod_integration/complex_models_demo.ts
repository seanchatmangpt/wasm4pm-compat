import { 
    BpmnProcessSchema, 
    PowlSchema, 
    ProcessTreeSchema, 
    XesLogSchema,
    EvidenceSchema,
    LossReportSchema,
    CompatDiagnosticSchema
} from './zod_schemas';

/**
 * Demo of complex process models and interchange shapes.
 */

function demoBpmn() {
    console.log("--- BPMN Validation ---");
    const process = {
        nodes: [
            { id: "start", name: "Start Event", kind: "StartEvent" },
            { id: "task1", name: "Do Work", kind: "Task" },
            { id: "end", name: "End Event", kind: "EndEvent" }
        ],
        edges: [
            { id: "e1", source: "start", target: "task1" },
            { id: "e2", source: "task1", target: "end" }
        ],
        lanes: []
    };
    BpmnProcessSchema.parse(process);
    console.log("✅ BPMN Process Validated");
}

function demoPowl() {
    console.log("\n--- POWL Validation ---");
    const powl = {
        nodes: [
            { id: 0, kind: "Atom(A)" },
            { id: 1, kind: "Atom(B)" },
            { id: 2, kind: "PartialOrder([0, 1])" }
        ],
        edges: [
            { from: 0, to: 1 }
        ],
        root: 2
    };
    PowlSchema.parse(powl);
    console.log("✅ POWL Model Validated");
}

function demoProcessTree() {
    console.log("\n--- Process Tree Validation ---");
    const tree = {
        nodes: [
            { kind: "Activity", label: "A" },
            { kind: "Activity", label: "B" },
            { kind: "Operator", children: [0, 1] } // e.g. Sequence
        ],
        root: 2
    };
    ProcessTreeSchema.parse(tree);
    console.log("✅ Process Tree Validated");
}

function demoXesInterchange() {
    console.log("\n--- XES Interchange Validation ---");
    const xes = {
        name: "Lifting Demo",
        extensions: [
            { name: "Concept", prefix: "concept", uri: "http://..." }
        ],
        traces: [
            {
                name: "case_1",
                events: [
                    { attributes: { "concept:name": "A", "time:timestamp": "2026-06-09T00:00:00Z" } }
                ]
            }
        ]
    };
    XesLogSchema.parse(xes);
    console.log("✅ XES Interchange Log Validated");
}

function demoEvidenceEnvelope() {
    console.log("\n--- Evidence Envelope Validation ---");
    const evidence = {
        inner: { activity: "A" },
        state: "Admitted",
        witness: "Ocel20"
    };
    EvidenceSchema.parse(evidence);
    console.log("✅ Evidence Envelope Validated");
}

try {
    console.log("🌟 wasm4pm-compat Advanced Shapes Demo 🌟\n");
    demoBpmn();
    demoPowl();
    demoProcessTree();
    demoXesInterchange();
    demoEvidenceEnvelope();
    console.log("\n🚀 All complex shapes validated successfully!");
} catch (e) {
    console.error("❌ Validation Failed:", e);
    process.exit(1);
}
