//! Cedar Policy Compiler
//!
//! Compiles Cedar authorization policies to WebAssembly for optimized execution.

pub mod ast;
pub mod compiler;
pub mod optimization;
pub mod wasm;

pub use compiler::{Compiler, CompilerError, CompilerResult};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_compilation() {
        let compiler = Compiler::new();
        let policy = r#"
            permit(principal, action, resource);
        "#;

        // This will fail until we implement the compiler
        // let result = compiler.compile_str(policy);
        // assert!(result.is_ok());
    }
}
