use quantum_compiler::QuantumCompiler;

fn main() {
    // Minimal deterministic source placeholder
    let source = r#"
        H 0
    "#;

    // Compile using the stable deterministic API
    let program = QuantumCompiler::compile(source);

    // Print deterministic debug representation
    // (quil_rs::Program does not implement Display)
    println!("{:#?}", program);
}
