use ai_optimizer::Optimizer;
use quantum_compiler::QuantumCompiler;
use quantum_executor::QuantumExecutor;
use std::fs;

fn main() {
    let source_code = fs::read_to_string("quantum_program.rs").expect("Failed to read file");

    // 1) Compile deterministically to Quil Program
    let compiled_program = QuantumCompiler::compile(&source_code);

    // 2) Deterministic optimization pass (text-based for v1.x)
    let compiled_text = format!("{:?}", compiled_program);
    let optimized_text = Optimizer::optimize(&compiled_text);

    println!("Optimized representation:\n{optimized_text}");

    // 3) Execute compiled program on simulated backend
    QuantumExecutor::execute(&compiled_program);
}
