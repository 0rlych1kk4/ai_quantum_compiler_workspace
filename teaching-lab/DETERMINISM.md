# Deterministic Compilation Guarantee

This teaching lab demonstrates **byte-for-byte deterministic quantum compilation**
using the Quantum Compiler for Rust (v1.x).

## What “Deterministic” Means Here

For this project, determinism means:

- Given the **same input source**
- Using the **same compiler version**
- On repeated executions

️the generated intermediate representation (Quil IR) is **identical at the byte level**.

There are:
- No heuristics
- No randomness
- No machine learning components
- No time- or environment-dependent behavior

---

## Reproducibility Verification

The following commands were executed from the repository root:

```bash
cargo run -p quantum_app --example simple_program > run1.out
cargo run -p quantum_app --example simple_program > run2.out
diff run1.out run2.out
```
