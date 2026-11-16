//! Byte-backed and scalar primitive values. Each named type owns one file.
pub mod string;
pub mod atomic_string;
pub mod b16;
pub mod af32;
pub mod ad64;
pub mod if64;
pub mod ld128;
pub mod aif64;
pub mod ald128;
mod history;
mod snapshot;
