//! Basic usage example of the Cedar policy compiler

use cedar_policy_compiler::{Compiler, CompilerResult};

fn main() -> CompilerResult<()> {
    // Example 1: Compile from a string
    let policy_text = r#"
        permit(principal, action, resource)
        when { principal.role == "admin" };
    "#;

    let compiler = Compiler::new();
    let wasm_bytes = compiler.compile_str(policy_text)?;

    println!("Compiled {} bytes of WebAssembly", wasm_bytes.len());

    // Write to file
    std::fs::write("policy.wasm", wasm_bytes)?;

    // Example 2: Compile with different optimization levels
    let compiler_unoptimized = Compiler::new().with_opt_level(0);
    let wasm_unoptimized = compiler_unoptimized.compile_str(policy_text)?;

    let compiler_optimized = Compiler::new().with_opt_level(2);
    let wasm_optimized = compiler_optimized.compile_str(policy_text)?;

    println!("Unoptimized: {} bytes", wasm_unoptimized.len());
    println!("Optimized: {} bytes", wasm_optimized.len());

    // Example 3: Compile from file
    let wasm_from_file = compiler.compile_file("examples/simple_policy.cedar")?;
    std::fs::write("simple_policy.wasm", wasm_from_file)?;

    println!("✓ All examples completed successfully!");

    Ok(())
}
