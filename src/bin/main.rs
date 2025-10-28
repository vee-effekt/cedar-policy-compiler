//! CLI for Cedar policy compiler

use cedar_policy_compiler::{Compiler, CompilerResult};

fn main() -> CompilerResult<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage(&args[0]);
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = if args.len() >= 4 && args[2] == "-o" {
        &args[3]
    } else {
        "output.wasm"
    };

    let opt_level = if args.contains(&"--opt".to_string()) {
        2
    } else {
        1
    };

    println!("Compiling Cedar policy: {}", input_file);
    println!("Output: {}", output_file);
    println!("Optimization level: {}", opt_level);

    let compiler = Compiler::new().with_opt_level(opt_level);
    let wasm_bytes = compiler.compile_file(input_file)?;

    std::fs::write(output_file, wasm_bytes)?;

    println!("✓ Compilation successful!");
    println!("Generated {} bytes of WebAssembly", std::fs::metadata(output_file)?.len());

    Ok(())
}

fn print_usage(program: &str) {
    println!("Cedar Policy Compiler");
    println!();
    println!("USAGE:");
    println!("    {} <input.cedar> [-o <output.wasm>] [--opt]", program);
    println!();
    println!("ARGS:");
    println!("    <input.cedar>       Cedar policy file to compile");
    println!();
    println!("OPTIONS:");
    println!("    -o <output.wasm>    Output file (default: output.wasm)");
    println!("    --opt               Enable aggressive optimizations");
    println!();
    println!("EXAMPLES:");
    println!("    {} policy.cedar", program);
    println!("    {} policy.cedar -o compiled.wasm", program);
    println!("    {} policy.cedar -o compiled.wasm --opt", program);
}
