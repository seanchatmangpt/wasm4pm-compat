---
name: Bug report
about: Type-law defect, fixture failure, or nightly regression
labels: bug
---

## What happened

<!-- Describe the unexpected behavior. If a compile-fail fixture passes when it should fail,
or a compile-pass fixture fails when it should pass, include the fixture name and .stderr. -->

## Rustc version

```
rustc --version --verbose
```

## Minimal reproduction

```rust
// Paste the smallest code that reproduces the issue
```

## Expected behavior

<!-- What law should be enforced? Which ALIVE gate criterion is affected? -->

## Actual behavior

<!-- What does rustc actually produce? Paste the error or lack thereof. -->
