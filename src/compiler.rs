//! Main compiler orchestration

use cedar_policy_core::ast::{Policy, Template};
use cedar_policy_core::parser::parse_policy_or_template;
use std::path::Path;
use thiserror::Error;

use crate::ast::lowering::LoweredModule;
use crate::wasm::codegen::WasmCodeGen;

pub type CompilerResult<T> = Result<T, CompilerError>;

#[derive(Debug, Error)]
pub enum CompilerError {
    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Compilation error: {0}")]
    CompilationError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("WASM encoding error: {0}")]
    WasmError(String),
}

/// Cedar to WebAssembly compiler
pub struct Compiler {
    /// Optimization level (0 = none, 1 = basic, 2 = aggressive)
    opt_level: u8,
}

impl Compiler {
    /// Create a new compiler with default settings
    pub fn new() -> Self {
        Self { opt_level: 1 }
    }

    /// Set optimization level (0-2)
    pub fn with_opt_level(mut self, level: u8) -> Self {
        self.opt_level = level.min(2);
        self
    }

    /// Compile a Cedar policy from a string
    pub fn compile_str(&self, source: &str) -> CompilerResult<Vec<u8>> {
        // Parse the policy (in cedar 4.4+, requires an optional PolicyID)
        let template = parse_policy_or_template(None, source)
            .map_err(|e| CompilerError::ParseError(format!("{:?}", e)))?;

        self.compile_template(&template)
    }

    /// Compile a parsed Cedar template (which may be a policy)
    pub fn compile_template(&self, template: &Template) -> CompilerResult<Vec<u8>> {
        // Convert template to policy for now
        // In v3.3, templates are the main AST type
        let ir = LoweredModule::from_template(template)
            .map_err(|e| CompilerError::CompilationError(e))?;

        // Step 2: Apply optimization passes
        let optimized_ir = if self.opt_level > 0 {
            crate::optimization::optimize(ir, self.opt_level)
        } else {
            ir
        };

        // Step 3: Generate WebAssembly
        let mut codegen = WasmCodeGen::new();
        let wasm_bytes = codegen
            .generate(&optimized_ir)
            .map_err(|e| CompilerError::WasmError(e))?;

        Ok(wasm_bytes)
    }

    /// Compile a Cedar policy from a file
    pub fn compile_file(&self, path: impl AsRef<Path>) -> CompilerResult<Vec<u8>> {
        let source = std::fs::read_to_string(path)?;
        self.compile_str(&source)
    }

    /// Compile a parsed Cedar policy
    pub fn compile_policy(&self, policy: &Policy) -> CompilerResult<Vec<u8>> {
        // Step 1: Lower Cedar AST to intermediate representation
        let ir = LoweredModule::from_policy(policy)
            .map_err(|e| CompilerError::CompilationError(e))?;

        // Step 2: Apply optimization passes
        let optimized_ir = if self.opt_level > 0 {
            crate::optimization::optimize(ir, self.opt_level)
        } else {
            ir
        };

        // Step 3: Generate WebAssembly
        let mut codegen = WasmCodeGen::new();
        let wasm_bytes = codegen
            .generate(&optimized_ir)
            .map_err(|e| CompilerError::WasmError(e))?;

        Ok(wasm_bytes)
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compiler_creation() {
        let compiler = Compiler::new();
        assert_eq!(compiler.opt_level, 1);

        let compiler = Compiler::new().with_opt_level(2);
        assert_eq!(compiler.opt_level, 2);

        let compiler = Compiler::new().with_opt_level(10);
        assert_eq!(compiler.opt_level, 2); // Clamped to max
    }
}
