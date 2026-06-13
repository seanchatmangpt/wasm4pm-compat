# Using Real Data from ~/wasm4pm

If you have real process data in the `~/wasm4pm` repository, you can validate it against these Zod schemas using the provided validation tool.

## Universal Validation Tool

We provide a specialized script for validating any external JSON file:

```bash
# Usage:
# npm run validate-path <path-to-json> <SchemaName>

# Example: Validate an OCEL log from wasm4pm
npm run validate-path ~/wasm4pm/exports/order_process.json OcelLog

# Example: Validate a Petri Net
npm run validate-path ~/wasm4pm/models/workflow.json PetriNet
```

## Manual Setup

Alternatively, you can link or copy data files manually:

1.  **Export Data**: Export your OCEL 2.0 or XES logs to JSON format in `~/wasm4pm`.
2.  **Link or Copy**: Symlink the data file into this directory or copy it:
    ```bash
    cp ~/wasm4pm/exports/my_log.json ./real_log.json
    ```
3.  **Run Custom Validation**: Edit `real_data_test.ts` to point to your file and run:
    ```bash
    npm run test-real
    ```

## Coverage

The generated Zod schemas in `bindings/zod_schemas.ts` cover the full surface of the `wasm4pm-compat` core, including:

- **OCEL 2.0**: Validates event-object relationships, object changes, and type definitions.
- **XES/EventLog**: Validates traces, events, and standard attributes.
- **Formal Models**: Validates Petri Nets, Directly-Follows Graphs (DFG), and Declare models.
- **Analytics**: Validates conformance checking results and token replay metrics.
- **Governance**: Validates cryptographic receipts and conformance verdicts.

## Structural Laws Enforced

Every Zod `parse()` or `safeParse()` call enforces the **Structural Laws** defined in the process intelligence ontology. This ensures that any data crossing the boundary into your TypeScript application is mathematically sound according to the `wasm4pm` standard.
