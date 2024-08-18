#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

// Export current sdk types for downstream users building with a different sdk
// version
pub use lumos_program;

lumos_program::declare_id!("unknown111111111111111111111111111111111111");
