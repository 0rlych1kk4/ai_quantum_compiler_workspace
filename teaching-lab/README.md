# Teaching & Research Lab

This folder contains minimal, hands-on labs for the **Quantum Compiler for Rust (Stable v1.x)**.

The labs are designed to demonstrate:
- **deterministic compilation** (same input → same output)
- **reproducible results** suitable for research and teaching
- an **inspectable compilation pipeline** for correctness-focused work

## Lab 01 — Reproducible Compilation

### Goal
Prove that compiling the same input multiple times produces **identical** Quil output.

### Prerequisites
- Rust toolchain installed (`rustc`, `cargo`)
- This repository cloned locally

### Steps (high-level)
1. Build the workspace
2. Compile the example quantum program
3. Run compilation twice
4. Compare the generated Quil output
5. Confirm outputs are identical

### Example Commands

```bash
# Build the workspace
cargo build

# Run the example program
cargo run --example simple_program

# Run it again and compare output
cargo run --example simple_program
```
### Why this matters
Many quantum toolchains apply heuristic or non-deterministic transformations that make results hard to reproduce.
This project’s stable v1.x line preserves **strict determinism**, enabling:
- reproducible experiments
- regression testing for compiler passes
- teaching and research workflows that require stable outputs
