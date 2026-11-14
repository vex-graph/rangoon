//! rangoon — the Rust vocabulary and library surface of the vexgraph ecosystem.
//!
//! The C floor is `lib` (`vex.h`, `lang/primitive.h`); this crate is its peer on
//! the Rust side, not a second floor. It adds no behavior to the scalar
//! vocabulary — it projects the names the C floor already owns, so a C header
//! and a Rust signature read as the same contract.

pub mod primitive;

pub use primitive::{any, bytes, const_any, d64, fnptr};
