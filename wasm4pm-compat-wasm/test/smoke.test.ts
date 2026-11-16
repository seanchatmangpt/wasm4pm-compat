// Non-vacuous smoke tests: instantiate the REAL jco-generated WASM component
// (dist/compat-ocel.wasm via dist/compat-ocel.js) and assert named-law results
// against the shared fixtures in wasm4pm-compat-ts/test/fixtures/ocel/.
//
// The TS fixtures carry epoch-millis `time` fields; wasm4pm compat's Rust
// OCEL type deserializes `time` as an RFC 3339 DateTime, so the loader
// normalizes numeric `time` to ISO strings — mutation semantics (relationship
// structure) are untouched. Same normalization as the component repo's
// smoke-test.mjs.
import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { dirname, join } from 'node:path';
import { ocelV2 } from '../dist/compat-ocel.js';

const HERE = dirname(fileURLToPath(import.meta.url));
const FIXTURES = join(HERE, '../../wasm4pm-compat-ts/test/fixtures/ocel/');

const load = (f: string): string => {
  const doc = JSON.parse(readFileSync(join(FIXTURES, f), 'utf8')) as unknown;
  const walk = (n: unknown): unknown => {
    if (Array.isArray(n)) return n.map(walk);
    if (n && typeof n === 'object') {
      const out: Record<string, unknown> = {};
      for (const [k, v] of Object.entries(n)) {
        out[k] =
          k === 'time' && typeof v === 'number'
            ? new Date(v).toISOString()
            : walk(v);
      }
      return out;
    }
    return n;
  };
  return JSON.stringify(walk(doc));
};

const refusalOf = (fn: () => unknown): string => {
  try {
    fn();
    return '';
  } catch (e) {
    return String(
      (e as { payload?: unknown })?.payload ?? (e as Error).message ?? e,
    );
  }
};

test('admitOcelJson accepts the valid OCEL v2 fixture', () => {
  const verdict = ocelV2.admitOcelJson(load('ocel_v2_valid.json'));
  assert.equal(verdict, 'admitted');
});

test('admitOcelJson refuses dangling e2o with DanglingEventObjectLink', () => {
  const err = refusalOf(() =>
    ocelV2.admitOcelJson(load('ocel_v2_mutation_dangling_e2o.json')),
  );
  assert.ok(
    err.includes('DanglingEventObjectLink'),
    `expected DanglingEventObjectLink, got: ${err}`,
  );
});

test('admitOcelJson refuses empty e2o with EmptyEventObjectLinks', () => {
  const err = refusalOf(() =>
    ocelV2.admitOcelJson(load('ocel_v2_mutation_empty_e2o.json')),
  );
  assert.ok(
    err.includes('EmptyEventObjectLinks'),
    `expected EmptyEventObjectLinks, got: ${err}`,
  );
});

test('parseOcelJson refuses invalid JSON with a non-empty error', () => {
  const err = refusalOf(() => ocelV2.parseOcelJson('{not json'));
  assert.notEqual(err, '');
});

test('validateOcelLocality refuses the locality fixture with ViolatesLocalityPrinciple', () => {
  // ocel_v2_mutation_locality.json re-points p2's team reference from t1 to
  // t2 (implicit deletion of (t1, p2)); the Gianola 2026 locality gate now
  // reachable from JSON refuses it by name.
  const err = refusalOf(() =>
    ocelV2.validateOcelLocality(
      load('ocel_v2_mutation_locality.json'),
      'team',
      'employee',
    ),
  );
  assert.ok(
    err.includes('ViolatesLocalityPrinciple'),
    `expected ViolatesLocalityPrinciple, got: ${err}`,
  );
});

test('validateOcelLocality accepts a stable team reference', () => {
  // Same fixture shape with e2 kept under t1: lawful, returns normally.
  const stable = JSON.stringify({
    eventTypes: [{ name: 'create team' }],
    objectTypes: [{ name: 'team' }, { name: 'employee' }],
    events: [
      {
        id: 'e1', type: 'create team', time: '2024-06-09T00:00:00Z',
        relationships: [
          { objectId: 't1', qualifier: 'team' },
          { objectId: 'p2', qualifier: 'employee' },
        ],
      },
      {
        id: 'e2', type: 'create team', time: '2024-06-09T00:01:00Z',
        relationships: [
          { objectId: 't1', qualifier: 'team' },
          { objectId: 'p2', qualifier: 'employee' },
        ],
      },
    ],
    objects: [
      { id: 't1', type: 'team' },
      { id: 'p2', type: 'employee' },
    ],
  });
  assert.doesNotThrow(() =>
    ocelV2.validateOcelLocality(stable, 'team', 'employee'),
  );
});

test('validateOcelJson: valid fixture clean, dangling fixture reports DANGLING_E2O', () => {
  const valid = ocelV2.validateOcelJson(load('ocel_v2_valid.json'));
  assert.deepEqual(valid, []);
  const dangling = ocelV2.validateOcelJson(
    load('ocel_v2_mutation_dangling_e2o.json'),
  );
  assert.ok(
    JSON.stringify(dangling).includes('DANGLING_E2O'),
    `expected DANGLING_E2O issue, got: ${JSON.stringify(dangling)}`,
  );
  for (const issue of dangling) {
    assert.equal(typeof issue.code, 'string');
    assert.equal(typeof issue.message, 'string');
  }
});

test('strict.checkStrictBoundary passes a fully-attested boundary and refuses a missing loss policy', async () => {
  const { strict } = await import('../dist/compat-ocel.js');
  const fullyAttested = (
    kind:
      | 'exports-format'
      | 'imports-format'
      | 'emits-events'
      | 'claims-receipt',
    name: string,
  ) => ({
    kind,
    name,
    hasWitness: true,
    hasRoundTripFixture: true,
    hasLossPolicy: true,
    hasRefusalPath: true,
    hasConformanceFields: true,
    hasReceiptShape: true,
    hasCertificationMapping: true,
    exportsRawEvidence: false,
    hiddenPmGrowth: false,
  });
  const ok = strict.checkStrictBoundary(
    fullyAttested('exports-format', 'xes-out'),
  );
  assert.equal(ok, true);
  const bad = fullyAttested('exports-format', 'xes-out');
  bad.hasLossPolicy = false;
  const err = refusalOf(() => strict.checkStrictBoundary(bad));
  assert.ok(
    err.includes('MissingLossPolicy'),
    `expected MissingLossPolicy, got: ${err}`,
  );
});
