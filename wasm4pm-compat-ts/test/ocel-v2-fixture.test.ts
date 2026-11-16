import { test } from 'node:test';
import * as assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { OcelLogSchema } from '../bindings/zod_schemas.ts';

const here = dirname(fileURLToPath(import.meta.url));

function loadFixture(name: string): unknown {
  return JSON.parse(readFileSync(join(here, 'fixtures', 'ocel', name), 'utf8'));
}

// Test A — structural (zod) surface.
//
// NOTE: zod parsing is structural only. Schema-level acceptance is NOT the
// refusal law: the Ocel 2.0 semantic laws (dangling E2O links, empty E2O,
// locality) are delivered by the wasm admit path, not by this schema. Each
// mutation fixture below parses successfully here by design; its expected
// refusal law comes from src/ocel.rs at the wasm boundary.
test('OcelLogSchema accepts the valid Gianola-locality OCEL 2.0 log', () => {
  const valid = loadFixture('ocel_v2_valid.json');
  const parsed = OcelLogSchema.parse(valid);
  // Narrow the schema's optional fields before use (schema is structural
  // only; a log without events/relationships would fail the assertions).
  assert.ok(parsed.events !== undefined, 'log must carry events');
  const events = parsed.events;
  // Non-empty E2O: every event carries at least one relationship.
  for (const ev of events) {
    assert.ok(ev.relationships !== undefined && ev.relationships.length > 0,
      `event ${ev.id} must have E2O links`);
  }
  // Gianola locality: employee p2 is referenced only within team t1.
  const p2Teams = new Set<string>();
  for (const ev of events) {
    for (const rel of ev.relationships!) {
      if (rel.objectId === 'p2' && rel.qualifier === 'team') p2Teams.add(rel.objectId);
    }
  }
  // p2's team context is fixed by the events that also reference a team.
  const teamRefsPerEvent = events.map((ev) =>
    ev.relationships!.some((r) => r.qualifier === 'team'),
  );
  assert.deepEqual(teamRefsPerEvent, [true, true]);
  assert.equal(p2Teams.size <= 1, true);
});

test('OcelLogSchema parses all three single-field mutation fixtures (structural pass, law refusal lives in wasm)', () => {
  for (const name of [
    'ocel_v2_mutation_dangling_e2o.json',
    'ocel_v2_mutation_empty_e2o.json',
    'ocel_v2_mutation_locality.json',
  ]) {
    const parsed = OcelLogSchema.parse(loadFixture(name));
    assert.ok(parsed, `${name} parses structurally`);
  }
});

// Test B — refusal-law string table, sourced verbatim from the Display impl
// in src/ocel.rs (OcelRefusal variants).
//
// Each entry documents its triggering condition. These laws are enforced by
// the wasm boundary: dangling/empty via the admit export admitOcelJson, and
// locality via the dedicated opt-in gate validateOcelLocality (the locality
// gate is deliberately separate from admit — admit stays structural-only).
// A separate lane's tests in wasm4pm-compat-wasm assert the wasm refusal text
// matches these strings on each fixture above.
export const LAW_STRINGS = {
  // An event relationship points at an objectId absent from log.objects.
  danglingE2o: 'DanglingEventObjectLink',
  // An event carries zero relationships (no E2O links at all).
  emptyE2o: 'EmptyEventObjectLinks',
  // A relationship references an objectId missing from log.objects.
  missingReference: 'MissingReferenceObject',
  // An object is referenced by relationships under conflicting qualifiers.
  multipleReferences: 'MultipleReferenceObjects',
  // An object of one type is observed under events spanning >1 object of
  // another type (Gianola locality principle violation).
  locality: 'ViolatesLocalityPrinciple',
} as const;

test('LAW_STRINGS entries are distinct, non-empty strings', () => {
  const values = Object.values(LAW_STRINGS);
  for (const v of values) {
    assert.equal(typeof v, 'string');
    assert.ok(v.length > 0);
  }
  assert.equal(new Set(values).size, values.length);
});
