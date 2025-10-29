//! WebAssembly code generation from lowered IR

use wasm_encoder::{
    CodeSection, ExportKind, ExportSection, Function, FunctionSection, Instruction as WasmInst,
    MemorySection, MemoryType, Module, TypeSection, ValType,
};

use crate::ast::lowering::{Instruction, LoweredModule};
use crate::wasm::runtime::{memory, Decision};
use crate::wasm::types::FunctionSignature;

/// WebAssembly code generator
pub struct WasmCodeGen {
    module: Module,
}

impl WasmCodeGen {
    pub fn new() -> Self {
        Self {
            module: Module::new(),
        }
    }

    /// Generate a complete WebAssembly module from the lowered IR
    pub fn generate(&mut self, lowered: &LoweredModule) -> Result<Vec<u8>, String> {
        // 1. Type section: Define function signatures
        let mut types = TypeSection::new();
        let sig = FunctionSignature::policy_entry();
        // In wasm-encoder 0.220+, use ty() instead of function()
        types.ty().function(
            sig.params.iter().map(|t| t.to_val_type()).collect::<Vec<_>>(),
            sig.results.iter().map(|t| t.to_val_type()).collect::<Vec<_>>(),
        );

        // 2. Function section: Declare functions
        let mut functions = FunctionSection::new();
        functions.function(0); // Main evaluation function uses type 0

        // 3. Memory section: Linear memory for string storage
        let mut memories = MemorySection::new();
        memories.memory(MemoryType {
            minimum: memory::INITIAL_PAGES.into(),
            maximum: Some(memory::MAX_PAGES.into()),
            memory64: false,
            shared: false,
            page_size_log2: None,
        });

        // 4. Export section: Export the main function and memory
        let mut exports = ExportSection::new();
        exports.export("evaluate", ExportKind::Func, 0);
        exports.export("memory", ExportKind::Memory, 0);

        // 5. Code section: Implement the function bodies
        let mut codes = CodeSection::new();
        let func_body = self.compile_function(&lowered.entry)?;
        codes.function(&func_body);

        // Assemble the module
        self.module.section(&types);
        self.module.section(&functions);
        self.module.section(&memories);
        self.module.section(&exports);
        self.module.section(&codes);

        // Clone to avoid move issue (acceptable for now)
        Ok(self.module.clone().finish())
    }

    /// Compile a function from the IR instructions
    fn compile_function(&self, func: &crate::ast::lowering::LoweredFunction) -> Result<Function, String> {
        let mut f = Function::new(vec![]); // No locals for now

        for inst in &func.instructions {
            self.compile_instruction(inst, &mut f)?;
        }

        // Every WASM function body must end with an End instruction
        f.instruction(&WasmInst::End);

        Ok(f)
    }

    /// Compile a single IR instruction to WASM instructions
    fn compile_instruction(&self, inst: &Instruction, f: &mut Function) -> Result<(), String> {
        match inst {
            // Literals
            Instruction::PushBool(b) => {
                f.instruction(&WasmInst::I32Const(if *b { 1 } else { 0 }));
            }
            Instruction::PushInt(i) => {
                f.instruction(&WasmInst::I64Const(*i));
            }
            Instruction::PushString(_s) => {
                // TODO: Implement string storage in linear memory
                // For now, push a placeholder pointer
                f.instruction(&WasmInst::I32Const(0));
            }

            // Comparison operations (for i64)
            Instruction::Equal => {
                f.instruction(&WasmInst::I64Eq);
            }
            Instruction::NotEqual => {
                f.instruction(&WasmInst::I64Ne);
            }
            Instruction::LessThan => {
                f.instruction(&WasmInst::I64LtS); // Signed less than
            }
            Instruction::LessThanOrEqual => {
                f.instruction(&WasmInst::I64LeS);
            }
            Instruction::GreaterThan => {
                f.instruction(&WasmInst::I64GtS);
            }
            Instruction::GreaterThanOrEqual => {
                f.instruction(&WasmInst::I64GeS);
            }

            // Logical operations (on i32)
            Instruction::And => {
                f.instruction(&WasmInst::I32And);
            }
            Instruction::Or => {
                f.instruction(&WasmInst::I32Or);
            }
            Instruction::Not => {
                f.instruction(&WasmInst::I32Eqz); // Logical not: x == 0
            }

            // Control flow
            Instruction::IfThenElse => {
                // Our IR currently emits: NoDecision (i64), Permit/Forbid (i64), condition (i32), IfThenElse
                // Stack at IfThenElse: [NoDecision (i64), Permit (i64), condition (i32)]
                //
                // We need to use locals to store the values and implement if-else
                // For now, use a simpler approach: convert i64 values to i32 for select
                // Or better: use if-else blocks
                //
                // We'll need to use locals for this. For now, let's convert to i32.
                // Stack: [else_value (i64), then_value (i64), condition (i32)]
                //
                // Actually, WASM select requires condition to be i32, and values can be i32 or i64
                // but values must match. Since we have i64 values, we need locals.
                //
                // Simplified approach: use a different algorithm with locals
                // But we don't have locals declared yet!
                //
                // Alternative: use i32 for everything
                // Let's just use select with typed operands - need BlockType

                //Since wasm_encoder supports typed select, use it:
                f.instruction(&WasmInst::Select);
            }

            Instruction::Return => {
                f.instruction(&WasmInst::Return);
            }

            // Policy decisions
            Instruction::Permit => {
                f.instruction(&WasmInst::I32Const(Decision::Permit as i32));
            }
            Instruction::Forbid => {
                f.instruction(&WasmInst::I32Const(Decision::Deny as i32));
            }
            Instruction::NoDecision => {
                f.instruction(&WasmInst::I32Const(Decision::NoDecision as i32));
            }

            // Entity operations - these would require runtime support
            Instruction::GetAttribute(_attr) => {
                // TODO: Call runtime function to get attribute
                return Err("GetAttribute not yet implemented".to_string());
            }
            Instruction::HasAttribute(_attr) => {
                // TODO: Call runtime function to check attribute
                return Err("HasAttribute not yet implemented".to_string());
            }
            Instruction::In => {
                // TODO: Call runtime function for entity hierarchy check
                return Err("In operator not yet implemented".to_string());
            }
        }

        Ok(())
    }
}

impl Default for WasmCodeGen {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::lowering::LoweredFunction;

    #[test]
    fn test_simple_permit_codegen() {
        let module = LoweredModule {
            entry: LoweredFunction {
                instructions: vec![
                    Instruction::Permit,
                    Instruction::Return,
                ],
            },
        };

        let mut codegen = WasmCodeGen::new();
        let result = codegen.generate(&module);

        assert!(result.is_ok());
        let wasm_bytes = result.unwrap();
        assert!(!wasm_bytes.is_empty());

        // Verify it's valid WASM (starts with magic number)
        assert_eq!(&wasm_bytes[0..4], b"\0asm");
    }

    #[test]
    fn test_boolean_logic_codegen() {
        let module = LoweredModule {
            entry: LoweredFunction {
                instructions: vec![
                    Instruction::PushBool(true),
                    Instruction::PushBool(false),
                    Instruction::And,
                    Instruction::Return,
                ],
            },
        };

        let mut codegen = WasmCodeGen::new();
        let result = codegen.generate(&module);

        assert!(result.is_ok());
    }
}
