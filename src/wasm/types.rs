//! Cedar type to WebAssembly type mapping

use wasm_encoder::ValType;

/// Maps Cedar types to WebAssembly types
#[derive(Debug, Clone, Copy)]
pub enum WasmType {
    /// Boolean (i32 in WASM: 0 = false, 1 = true)
    Bool,
    /// Integer (i64)
    Int,
    /// String (represented as i32 pointer + i32 length in linear memory)
    String,
    /// Entity reference (i32 index into entity table)
    Entity,
}

impl WasmType {
    /// Convert to wasm-encoder ValType
    pub fn to_val_type(self) -> ValType {
        match self {
            WasmType::Bool => ValType::I32,
            WasmType::Int => ValType::I64,
            WasmType::String => ValType::I32, // Pointer to string in linear memory
            WasmType::Entity => ValType::I32,  // Index into entity table
        }
    }
}

/// Function signature in WebAssembly
#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub params: Vec<WasmType>,
    pub results: Vec<WasmType>,
}

impl FunctionSignature {
    pub fn new(params: Vec<WasmType>, results: Vec<WasmType>) -> Self {
        Self { params, results }
    }

    /// Create signature for the main policy evaluation function
    /// Input: none (uses global state)
    /// Output: i32 (-1 = no decision, 0 = deny, 1 = permit, 2 = error)
    pub fn policy_entry() -> Self {
        Self {
            params: vec![],
            results: vec![WasmType::Bool], // Use Bool which maps to i32
        }
    }
}
