//! Individual optimization passes

use crate::ast::lowering::{Instruction, LoweredModule};

/// Constant folding: evaluate constant expressions at compile time
pub fn constant_folding(module: LoweredModule) -> LoweredModule {
    // TODO: Implement constant folding
    // For example: PushBool(true), PushBool(false), And -> PushBool(false)
    module
}

/// Dead code elimination: remove unreachable code
pub fn dead_code_elimination(mut module: LoweredModule) -> LoweredModule {
    // Remove instructions after Return
    if let Some(return_idx) = module
        .entry
        .instructions
        .iter()
        .position(|inst| matches!(inst, Instruction::Return))
    {
        module.entry.instructions.truncate(return_idx + 1);
    }

    module
}

/// Instruction combining: combine multiple instructions into more efficient forms
pub fn instruction_combining(module: LoweredModule) -> LoweredModule {
    // TODO: Implement peephole optimizations
    // For example: Not, Not -> remove both
    module
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::lowering::LoweredFunction;

    #[test]
    fn test_dead_code_elimination() {
        let module = LoweredModule {
            entry: LoweredFunction {
                instructions: vec![
                    Instruction::PushBool(true),
                    Instruction::Return,
                    Instruction::PushBool(false), // Dead code
                ],
            },
        };

        let optimized = dead_code_elimination(module);
        assert_eq!(optimized.entry.instructions.len(), 2);
    }
}
