# Quantum Compiler for Rust (Stable v1.x)

A deterministic and extensible quantum compiler framework in Rust, designed for reproducibility, research, and future AI-driven optimization.

Welcome to the Quantum Compiler for Rust, a modular and deterministic
Rust workspace for compiling, optimizing, and executing quantum programs
using the Quil intermediate representation.

This project provides a stable architectural foundation for future
AI-assisted and hardware-aware quantum compilation, while maintaining
strictly deterministic behavior in v1.x.

---

## Teaching & Research Use

The project includes a teaching lab that demonstrates deterministic quantum
compilation with byte-for-byte reproducibility across repeated runs.

---

## Stability Status

Current version: v1.x (Stable)

This release guarantees:
- Deterministic compilation and optimization
- Stable public APIs across v1.x
- No placeholder or misleading functionality
- Fully simulated execution backend

No machine learning or AI models are used in v1.x.  
AI-based optimization is a planned future capability.

---

## Features (v1.x)

- Deterministic Quantum Compilation
  - Compiles Rust-based quantum source input into Quil IR
  - Same input always produces the same output

- Pluggable Optimization Pass Framework
  - Trait-based optimization pipeline
  - Supports custom user-defined passes

- Built-in Optimization Passes
  - Whitespace normalization
  - Consecutive duplicate elimination

- Simulated Quantum Execution Backend
  - Executes compiled Quil programs in a safe, predictable environment
  - Designed for extensibility to real backends

- Modular Rust Workspace
  - Clear separation of concerns
  - Easy to extend without breaking APIs

---

## Deterministic Optimization Pipeline

The compiler includes a deterministic optimization pipeline designed for stability, reproducibility, and extensibility.

Unlike heuristic or probabilistic optimizers, this pipeline guarantees:
- the same input always produces the same output
- no randomness or ML heuristics are used in v1.x
- behavior remains predictable for testing and research

---

### Architecture

The optimizer is implemented as a pluggable pass pipeline:

```rust
pub trait OptimizationPass {
    fn name(&self) -> &'static str;
    fn run(&self, lines: Vec<String>) -> Vec<String>;
}
```

Multiple passes are composed into a pipeline:

```rust
let optimizer = Optimizer::new();
let optimized = optimizer.optimize(input);
```
---
## Built-in Passes
- Whitespace Normalization
  - Removes empty lines
  - Trims formatting noise
- Consecutive Duplicate Elimination
  - Removes redundant repeated instructions
  - Ensures minimal deterministic representation
---
## Custom Optimization Passes
Users can define their own optimization logic:

```rust
struct MyPass;

impl OptimizationPass for MyPass {
    fn name(&self) -> &'static str {
        "my_pass"
    }

    fn run(&self, lines: Vec<String>) -> Vec<String> {
        lines
    }
}

let optimizer = Optimizer::with_passes(vec![
    Box::new(MyPass),
]);

```
---
## Design Goals

 - Deterministic execution (v1.x guarantee)
 - Extensible optimization framework
 - Research-friendly architecture
 - Safe Rust (no unsafe)

---

## Future Roadmap
- The optimization pipeline is designed to support:
 - AI-driven optimization passes
 - Hardware-aware transformations
 - Cost-based optimization strategies
 - Optional runtime plugin loading
 - These capabilities will be introduced in future versions without breaking v1.x stability guarantees. 

---

## Usage

```rust
let optimizer = Optimizer::new();
let optimized = optimizer.optimize(&compiled_text);
```
## Why This Matters
- Quantum compilation and optimization are critical for:
 - reducing circuit depth
 - improving execution fidelity
 - adapting programs to hardware constraints
- This project introduces a deterministic, extensible optimization framework that enables:
 - reproducible experimentation
 - safe integration of advanced optimization techniques
 - future AI-assisted optimization workflows
It serves as a foundation for next-generation quantum compilers combining
systems engineering, formal guarantees, and intelligent optimization.
---

## Project Structure

```plaintext
ai_quantum_compiler_workspace/
│── Cargo.toml            # Workspace manifest
│── quantum_program.rs    # Example Rust quantum program
│
├── ai_optimizer/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs        # Deterministic optimizer (v1.x stable)
│
├── quantum_compiler/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs        # Rust → Quil compiler (deterministic)
│
├── quantum_executor/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs        # Simulated execution backend
│
├── quantum_app/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs       # End-to-end application entry point

```
---
