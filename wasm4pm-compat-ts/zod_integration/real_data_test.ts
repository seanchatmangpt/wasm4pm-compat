import fs from 'fs';
import { OcelLogSchema, OcelType, OcelRelationship } from './zod_schemas';

/**
 * Real-world data test using OCEL 2.0 JSON from wasm4pm repository.
 */

async function runTest() {
    console.log("🚀 Starting Real Data Validation Test (OCEL 2.0 World Fixture)...");

    // Loading the 'world/ocel-v2.json' fixture copied from wasm4pm
    const rawData = JSON.parse(fs.readFileSync('./data/ocel.json', 'utf8'));

    console.log(`📊 Input: ${rawData.events.length} events, ${rawData.objects.length} objects.`);

    const result = OcelLogSchema.safeParse(rawData);

    if (result.success) {
        console.log("✅ SUCCESS: Data conforms to OCEL 2.0 structural laws.");
        
        // Example of accessing typed data
        const log = result.data;
        console.log("\nSummary of validated data:");
        console.log(`- Event Types: ${log.eventTypes.map((et: OcelType) => et.name).join(", ")}`);
        console.log(`- Object Types: ${log.objectTypes.map((ot: OcelType) => ot.name).join(", ")}`);

        const firstEvent = log.events[0];
        console.log(`- First Event: ${firstEvent.type} at ${new Date(firstEvent.time).toISOString()}`);
        console.log(`  Relationships: ${firstEvent.relationships.map((r: OcelRelationship) => `${r.qualifier}->${r.objectId}`).join(", ")}`);
    } else {
        console.error("❌ FAILURE: Structural Law Violation Detected!");
        console.error(JSON.stringify(result.error.format(), null, 2));
        process.exit(1);
    }
}

runTest().catch(err => {
    console.error("Fatal Error:", err);
    process.exit(1);
});
