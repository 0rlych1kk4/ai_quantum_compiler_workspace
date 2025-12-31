use quantum_compiler::QuantumCompiler;

#[test]
fn compile_is_deterministic() {
    let src = "fn quantum_main() { println!(\"hi\"); }";
    let a = QuantumCompiler::compile(src);
    let b = QuantumCompiler::compile(src);

    assert_eq!(format!("{:?}", a), format!("{:?}", b));
}
