use quantum_compiler::compile_to_quil;

fn main() {
    // Minimal example: a simple quantum program description
    let program = r#"
        H 0
        CNOT 0 1
    "#;

    let quil = compile_to_quil(program).expect("Compilation failed");
    println!("{}", quil);
}
