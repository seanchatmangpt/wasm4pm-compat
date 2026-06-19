import { test } from 'node:test';
import * as assert from 'node:assert/strict';
import { ConformanceResultSchema } from '../index.ts';

test('ConformanceResultSchema accepts a valid object', () => {
  const valid = {
    deviating_traces: 3,
    fitness: 0.97,
    fitting_traces: 97,
    precision: 0.88,
    total_traces: 100,
  };
  const parsed = ConformanceResultSchema.parse(valid);
  assert.equal(parsed.fitness, 0.97);
  assert.equal(parsed.total_traces, 100);
});

test('ConformanceResultSchema accepts a valid object without optional precision', () => {
  const valid = {
    deviating_traces: 0,
    fitness: 1,
    fitting_traces: 100,
    total_traces: 100,
  };
  assert.doesNotThrow(() => ConformanceResultSchema.parse(valid));
});

test('ConformanceResultSchema rejects an invalid object', () => {
  const invalid = {
    deviating_traces: 'three', // wrong type
    fitness: 0.97,
    fitting_traces: 97,
    total_traces: 100,
  };
  assert.throws(() => ConformanceResultSchema.parse(invalid));
});
