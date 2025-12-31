use quantum_executor::QuantumExecutor;
use quil_rs::program::Program;

#[test]
fn executor_does_not_panic_on_empty_program() {
    let p = Program::new();
    QuantumExecutor::execute(&p);
}
