# Cedar Policy Compiler

A compiler that transforms Cedar authorization policies into WebAssembly for optimized execution.

## Overview

This compiler takes Cedar policies (parsed via `cedar-policy-core`) and compiles them to WebAssembly bytecode. This enables:

- **Performance**: Native-speed policy evaluation
- **Portability**: Run Cedar policies anywhere WASM runs
- **Optimization**: Apply compiler optimization passes
- **Ahead-of-time compilation**: Compile policies once, evaluate many times

## Architecture

```
Cedar Policy (text)
    ↓
Cedar AST (cedar-policy-core parser)
    ↓
Lowered IR (ast/lowering.rs)
    ↓
Optimization passes (optimization/)
    ↓
WASM CodeGen (wasm/codegen.rs)
    ↓
WebAssembly Module (.wasm)
```

## Usage

### As a Library

```rust
use cedar_policy_compiler::Compiler;

let policy_text = r#"
    permit(principal, action, resource)
    when { principal.role == "admin" };
"#;

let compiler = Compiler::new();
let wasm_module = compiler.compile_str(policy_text)?;

// Write to file or execute with wasmtime/wasmer
std::fs::write("policy.wasm", wasm_module)?;
```

### As a CLI

```bash
cargo run --bin cedar-compile -- input.cedar -o output.wasm
```

## Project Status

**Early Development** - API is unstable and subject to change.

## Dependencies

- `cedar-policy-core` v4.4.0 - Cedar AST and parser
- `wasm-encoder` v0.220 - WebAssembly encoding
- `wasmparser` v0.220 - WASM validation
- `wasmtime` v28.0 - WASM runtime (dev/testing)

### Requirements

- **Rust 1.85+** required for cedar-policy-core 4.4.0
- If you have an older Rust version, modify `Cargo.toml` to use cedar-policy-core 4.2.x instead

### Current Status

**Note**: This project is configured for cedar-policy-core 4.4.0 which requires Rust 1.85+.
The code structure is complete but has API compatibility fixes pending for cedar 4.4.0.

## License

Apache-2.0
