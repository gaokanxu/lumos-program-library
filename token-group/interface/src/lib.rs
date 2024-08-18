//! Crate defining the LPL Token Group Interface

#![deny(missing_docs)]
#![cfg_attr(not(test), forbid(unsafe_code))]

pub mod error;
pub mod instruction;
pub mod state;

/// Namespace for all programs implementing lpl-token-group
pub const NAMESPACE: &str = "lpl_token_group_interface";
