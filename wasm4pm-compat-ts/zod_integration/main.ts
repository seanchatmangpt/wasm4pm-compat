import fs from 'fs';
import path from 'path';
import { EventLogSchema, OcelLogSchema, PetriNetSchema } from './zod_schemas';

/**
 * Basic usage example using real data from wasm4pm ecosystem.
 */

function demoEventLog() {
    console.log("--- Event Log Validation Demo (Real Data) ---");

    // Using the sample created for compatibility testing
    const raw = fs.readFileSync('./sample_eventlog.json', 'utf8');
    const validLog = JSON.parse(raw);

    const result = EventLogSchema.safeParse(validLog);
    if (result.success) {
        console.log("✅ EventLog is valid!");
        console.log(`Parsed ${result.data.traces.length} traces from wasm4pm standard.`);
    } else {
        console.error("❌ EventLog is invalid:", result.error.format());
    }
}

function demoOcel() {
    console.log("\n--- OCEL 2.0 Validation Demo (Real Data) ---");

    // Loading real OCEL 2.0 world fixture
    const raw = fs.readFileSync('./data/ocel.json', 'utf8');
    const validOcel = JSON.parse(raw);

    const result = OcelLogSchema.safeParse(validOcel);
    if (result.success) {
        console.log("✅ OCEL Log is valid!");
        console.log(`Parsed ${result.data.events.length} events and ${result.data.objects.length} objects from 'ocel-v2.json'.`);
    } else {
        console.error("❌ OCEL Log is invalid:", result.error.format());
    }
}

function demoPetriNet() {
    console.log("\n--- Petri Net Validation Demo (Real Data) ---");

    // Loading real Workflow Net world fixture
    const raw = fs.readFileSync('./data/petri_net.json', 'utf8');
    const validNet = JSON.parse(raw);

    const result = PetriNetSchema.safeParse(validNet);
    if (result.success) {
        console.log("✅ Petri Net is valid!");
        console.log(`Net has ${result.data.places.length} places and ${result.data.transitions.length} transitions from 'wf-net.json'.`);
    } else {
        console.error("❌ Petri Net is invalid:", result.error.format());
    }
}

// Run demos
demoEventLog();
demoOcel();
demoPetriNet();
