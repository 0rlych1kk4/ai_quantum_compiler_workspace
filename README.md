# Quantum Compiler for Rust (Stable v1.x)

Welcome to the **Quantum Compiler for Rust**, a modular and deterministic
Rust workspace for compiling, optimizing, and executing quantum programs
using the Quil intermediate representation.

This project provides a **stable architectural foundation** for future
AI-assisted and hardware-aware quantum compilation, while maintaining
strictly deterministic behavior in v1.x.

---

## Stability Status

**Current version:** v1.x (Stable)

This release guarantees:
- Deterministic compilation and optimization
- Stable public APIs across v1.x
- No placeholder or misleading functionality
- Fully simulated execution backend

> ️ No machine learning or AI models are used in v1.x.
> AI-based optimization is a **planned future capability**.

---

## Features (v1.x)

- **Deterministic Quantum Compilation**
  - Compiles Rust-based quantum source input into Quil IR
  - Same input always produces the same output

- **Deterministic Optimization Pass**
  - Stable optimization interface
  - Currently implemented as an identity transform (no heuristics, no ML)

- **Simulated Quantum Execution Backend**
  - Executes compiled Quil programs in a safe, predictable environment
  - Designed for extensibility to real backends in future versions

- **Modular Rust Workspace**
  - Clear separation of concerns
  - Easy to extend without breaking APIs

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

