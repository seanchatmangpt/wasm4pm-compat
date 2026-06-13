import fs from 'fs';
import * as schemas from './zod_schemas';

/**
 * Universal validation script for external data.
 * Usage: npm run validate-path <path-to-json> <SchemaName>
 * Example: npm run validate-path ~/wasm4pm/data/my_log.json OcelLog
 */

async function main() {
    const filePath = process.argv[2];
    const schemaName = process.argv[3];

    if (!filePath || !schemaName) {
        console.error("Usage: npm run validate-path <path-to-json> <SchemaName>");
        console.error("Available schemas:", Object.keys(schemas).filter(k => k.endsWith('Schema')).map(k => k.replace('Schema', '')));
        process.exit(1);
    }

    const fullSchemaName = schemaName.endsWith('Schema') ? schemaName : `${schemaName}Schema`;
    const schema = (schemas as Record<string, import('zod').ZodTypeAny>)[fullSchemaName];

    if (!schema) {
        console.error(`Error: Schema '${schemaName}' not found in bindings.`);
        process.exit(1);
    }

    try {
        console.log(`🔍 Validating '${filePath}' against '${fullSchemaName}'...`);
        const data = JSON.parse(fs.readFileSync(filePath, 'utf8'));
        
        const startTime = Date.now();
        const result = schema.safeParse(data);
        const duration = Date.now() - startTime;

        if (result.success) {
            console.log(`✅ VALID: Data satisfies all structural laws of '${schemaName}'.`);
            console.log(`⏱️  Validation time: ${duration}ms`);
        } else {
            console.error(`❌ INVALID: Structural law violations found!`);
            console.error(JSON.stringify(result.error.format(), null, 2));
            process.exit(1);
        }
    } catch (err) {
        const message = err instanceof Error ? err.message : String(err);
        console.error(`Fatal Error: ${message}`);
        process.exit(1);
    }
}

main();
