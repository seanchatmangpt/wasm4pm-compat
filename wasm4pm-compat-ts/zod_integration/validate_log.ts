import { z } from 'zod';
import { EventLogSchema } from './zod_schemas';

/**
 * Advanced validation example: adding refinements to generated schemas.
 * 
 * In this example, we extend the base EventLogSchema with domain-specific
 * laws that are checked at runtime.
 */

// Extend the generated schema with a custom refinement
const StrictEventLogSchema = EventLogSchema.refine((log) => {
    // Law: A log must have at least one trace
    if (log.traces.length === 0) return false;

    // Law: Each trace must have at least one event
    return log.traces.every(trace => trace.events.length > 0);
}, {
    message: "Strict Law Violation: Logs and traces must not be empty"
});

function validateExternalData(data: unknown) {
    console.log("--- Strict Validation Demo ---");

    const result = StrictEventLogSchema.safeParse(data);
    
    if (result.success) {
        console.log("✅ Data satisfies all structural and strict laws.");
    } else {
        console.error("❌ Validation Failed:");
        console.error(JSON.stringify(result.error.flatten(), null, 2));
    }
}

// Case 1: Empty traces (Invalid under strict law)
validateExternalData({
    traces: [{ id: "case_1", events: [] }],
    attributes: {}
});

// Case 2: Valid data
validateExternalData({
    traces: [{ 
        id: "case_2", 
        events: [{ activity: "action" }] 
    }],
    attributes: { "origin": "test" }
});
