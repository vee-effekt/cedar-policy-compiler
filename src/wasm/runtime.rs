//! Runtime support functions for compiled policies

/// Runtime decision values
#[repr(i32)]
pub enum Decision {
    Deny = 0,
    Permit = 1,
    Error = 2,
}

/// Runtime function indices
/// These are helper functions that will be included in the WASM module
pub mod runtime_functions {
    pub const STRING_EQ: u32 = 0;
    pub const GET_ATTRIBUTE: u32 = 1;
    pub const HAS_ATTRIBUTE: u32 = 2;
    pub const ENTITY_IN: u32 = 3;
}

/// Memory layout for the linear memory
pub mod memory {
    /// Initial memory size in WASM pages (64KB each)
    pub const INITIAL_PAGES: u32 = 1;

    /// Maximum memory size in WASM pages
    pub const MAX_PAGES: u32 = 16;

    /// String data starts at this offset
    pub const STRING_POOL_START: u32 = 0x1000;
}
