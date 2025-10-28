//! Optimization passes for the intermediate representation

pub mod passes;

use crate::ast::lowering::LoweredModule;

/// Apply optimization passes to the IR
pub fn optimize(module: LoweredModule, opt_level: u8) -> LoweredModule {
    let mut optimized = module;

    if opt_level >= 1 {
        // Basic optimizations
        optimized = passes::constant_folding(optimized);
        optimized = passes::dead_code_elimination(optimized);
    }

    if opt_level >= 2 {
        // Aggressive optimizations
        optimized = passes::instruction_combining(optimized);
    }

    optimized
}
