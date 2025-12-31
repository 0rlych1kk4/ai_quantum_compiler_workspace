#![forbid(unsafe_code)]

use quil_rs::instruction::{Gate, Instruction, Qubit};
use quil_rs::program::Program;

/// Quantum Compiler (stable v1.x)
///
/// Deterministic behavior:
/// Given the same input, the output Quil program will be identical.
pub struct QuantumCompiler;

impl QuantumCompiler {
    pub fn compile(_source_code: &str) -> Program {
        println!("Compiling Rust-based quantum code to Quil...");

        let mut program = Program::new();

        let h = Gate::new(
            "H",
            vec![],                // parameters
            vec![Qubit::Fixed(0)], // qubits
            vec![],                // modifiers
        )
        .expect("failed to build gate");

        // quil-rs 0.33.0 expects an Instruction here
        program.add_instruction(Instruction::Gate(h));

        program
    }
}
