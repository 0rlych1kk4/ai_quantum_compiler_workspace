use ai_optimizer::AIOptimizer;
use quantum_compiler::QuantumCompiler;
use quantum_executor::QuantumExecutor;
use quil_rs::program::Program;
use std::fs;

fn main() {
    let source_code = fs::read_to_string("quantum_program.rs").expect("Failed to read file");
    
    let compiled_circuit = QuantumCompiler::compile(&source_code);
    
    let device = tch::Device::cuda_if_available();
    let vs = tch::nn::VarStore::new(device);
    let optimizer = AIOptimizer::new(&vs.root());

    let optimized_circuit = optimizer.optimize(format!("{:?}", compiled_circuit));
    
    QuantumExecutor::execute(&compiled_circuit);
}

