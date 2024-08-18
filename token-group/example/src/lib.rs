//! Crate defining an example program for creating LPL token groups
//! using the LPL Token Group interface.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

pub mod processor;

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;
