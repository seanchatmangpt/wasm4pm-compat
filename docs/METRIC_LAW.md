# METRIC_LAW — Between01 and the Five Quality Dimensions

> Every conformance quality metric is bounded to [0, 1] at the **type level**
> via `Between01<NUM, DEN>`. An out-of-range metric is a compile error.

---

## The `Between01` bound

```rust
pub struct Between01<const NUM: u64, const DEN: u64>
where
    Require<{ NUM <= DEN }>: IsTrue,
    Require<{ DEN > 0 }>: IsTrue,
{
    _private: (),
}
```

Two compile-time constraints:

1. `NUM <= DEN` — the metric numerator must not exceed the denominator (value <= 1).
2. `DEN > 0` — the denominator must be positive (no division by zero).

Both bounds use `Require<{ EXPR }>: IsTrue` from `src/law.rs`. If either bound fails,
the compiler emits:

```
the trait bound `Require<false>: IsTrue` is not satisfied
```

This is the canonical Between01 law violation message.

---

## The `Metric<KIND, NUM, DEN>` type

```rust
pub struct Metric<const KIND: QualityMetricKind, const NUM: u64, const DEN: u64>
where
    Between01<NUM, DEN>: /* inhabited */;
```

`KIND` is a `ConstParamTy` enum variant from `src/law.rs`. The five quality
dimensions each have a type alias:

| Alias | KIND value | Description |
|-------|-----------|-------------|
| `FitnessConst<NUM, DEN>` | `QualityMetricKind::Fitness` | Fraction of log behavior reproducible by the model |
| `PrecisionConst<NUM, DEN>` | `QualityMetricKind::Precision` | Fraction of model behavior actually seen in the log |
| `F1Const<NUM, DEN>` | `QualityMetricKind::F1` | Harmonic mean of fitness and precision |
| `GeneralizationConst<NUM, DEN>` | `QualityMetricKind::Generalization` | Model's ability to generalize beyond the training log |
| `SimplicityConst<NUM, DEN>` | `QualityMetricKind::Simplicity` | Structural simplicity of the model |

---

## The five quality dimensions

These map directly to the classic four from van der Aalst's process mining canon,
plus generalization:

### 1. Fitness

> Does the model reproduce the behavior observed in the event log?

- A fitness of 1.0 means every trace in the log can be replayed by the model.
- A fitness of 0.0 means no trace in the log fits the model.
- Type alias: `FitnessConst<NUM, DEN>`

### 2. Precision

> Does the model avoid producing behavior not seen in the event log?

- A precision of 1.0 means the model allows only what the log contains.
- A precision of 0.0 means the model is a "flower model" allowing everything.
- Type alias: `PrecisionConst<NUM, DEN>`

### 3. F1 (Balanced quality)

> Harmonic mean of fitness and precision — balances the two competing forces.

- A high F1 requires both high fitness and high precision simultaneously.
- Type alias: `F1Const<NUM, DEN>`

### 4. Generalization

> Does the model generalize to unseen but lawful behavior?

- A high generalization score means the model is not overfitted to the training log.
- Type alias: `GeneralizationConst<NUM, DEN>`

### 5. Simplicity

> Is the model as simple as possible while still describing the process?

- A simplicity of 1.0 means no unnecessary constructs.
- Type alias: `SimplicityConst<NUM, DEN>`

---

## Compile-fail receipts (out-of-range rejections)

| Fixture | Law sealed |
|---------|-----------|
| `fitness_out_of_bounds_3_2.rs` | FitnessConst<3,2> violates NUM <= DEN — 3/2 > 1 |
| `fitness_num_gt_den.rs` | FitnessConst<3,2> rejected because NUM > DEN |
| `precision_out_of_bounds.rs` | PrecisionConst<NUM,DEN> requires NUM <= DEN |
| `precision_num_gt_den.rs` | PrecisionConst<5,3> rejected because 5/3 > 1 |
| `f1_out_of_bounds.rs` | F1Const<NUM,DEN> requires NUM <= DEN |
| `f1_num_gt_den.rs` | F1Const<2,1> rejected because 2/1 > 1 |
| `generalization_out_of_bounds.rs` | GeneralizationBoundsLaw — scores above 1.0 rejected |
| `generalization_out_of_bounds_8_7.rs` | GeneralizationConst<8,7> violates NUM <= DEN |
| `generalization_num_gt_den.rs` | GeneralizationConst<8,7> rejected because 8/7 > 1 |
| `simplicity_out_of_bounds.rs` | SimplicityBoundsLaw — simplicity above 1.0 rejected |
| `simplicity_out_of_bounds_10_9.rs` | SimplicityConst<10,9> violates NUM <= DEN |
| `simplicity_num_gt_den.rs` | SimplicityConst<10,9> rejected because 10/9 > 1 |
| `metric_out_of_bounds.rs` | MetricBoundsLaw — FitnessConst above 1.0 violates Between01 |
| `metric_den_zero.rs` | FitnessConst<1,0> rejected because DEN=0 |

---

## Compile-fail receipts (metric kind confusion)

| Fixture | Law sealed |
|---------|-----------|
| `fitness_as_precision.rs` | FitnessConst cannot be passed where PrecisionConst is required |
| `precision_as_f1.rs` | PrecisionConst cannot be passed where F1Const is required |

---

## Compile-pass receipts

| Fixture | Law proven |
|---------|-----------|
| `conformance_fitness_const_alias.rs` | FitnessConst type alias is well-formed |
| `conformance_fitness_runtime.rs` | Runtime fitness score is well-shaped |
| `conformance_fitness_precision_specific.rs` | Specific FitnessConst<3,4> and PrecisionConst<1,2> compile |
| `conformance_generalization_metric.rs` | GeneralizationConst well-formed |
| `conformance_generalization_nine_tenths.rs` | GeneralizationConst<9,10> compiles — 9/10 in [0,1] |
| `conformance_simplicity_metric.rs` | SimplicityConst well-formed |
| `conformance_simplicity_seven_eighths.rs` | SimplicityConst<7,8> compiles — 7/8 in [0,1] |
| `conformance_f1_zero_and_perfect.rs` | F1Const<0,1> and F1Const<1,1> both compile |
| `conformance_precision_f1_aliases.rs` | PrecisionConst and F1Const aliases both compile |
| `conformance_quality_profile_construction.rs` | QualityProfile with all five dimensions compiles |
| `law_between01_zero_one.rs` | Between01<0,1> and Between01<1,1> compile as boundary cases |

---

## This crate does not compute metrics

`Metric<KIND, NUM, DEN>` carries a score claim — it does not derive one.
Computing fitness requires token replay against a model. Computing precision
requires behavioral analysis. Those computations graduate to `wasm4pm`.

This crate certifies that a score handed to it is **well-shaped and in-bounds** —
nothing more.
