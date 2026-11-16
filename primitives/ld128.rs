//! DEFINITION: `ld128` is the 128-bit split `{ integer: i64, fraction: d64 }` projection.
//! OVERVIEW: `#[repr(C, align(8))] ld128 { integer: i64, fraction: d64 }`.
//! Preserves C ABI size (16), alignment (8), and field offsets (`integer@0`, `fraction@8`).

#![allow(non_camel_case_types)]

use core::mem::{align_of, offset_of, size_of};
use crate::annotation::overview;
use crate::primitive::d64;

/// `ld128` — 128-bit integer + fraction coordinate pair matching `lib/lang/primitive.h`.
#[repr(C, align(8))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[overview]
pub struct ld128 {
    pub integer: i64,
    pub fraction: d64,
}

const _: () = assert!(size_of::<ld128>() == 16);
const _: () = assert!(align_of::<ld128>() == 8);
const _: () = assert!(offset_of!(ld128, integer) == 0);
const _: () = assert!(offset_of!(ld128, fraction) == 8);

impl ld128 {
    /// Zero value `{ integer: 0, fraction: 0.0 }`.
    pub const fn zero() -> Self {
        Self { integer: 0, fraction: 0.0 }
    }

    /// Construct an `ld128` from 64-bit integer and fractional components.
    pub const fn new(integer: i64, fraction: d64) -> Self {
        Self { integer, fraction }
    }

    /// Return the signed 64-bit integer component.
    pub const fn integer(self) -> i64 {
        self.integer
    }

    /// Return the 64-bit binary64 fractional remainder.
    pub const fn fraction(self) -> d64 {
        self.fraction
    }

    /// Set the signed 64-bit integer component.
    pub fn set_integer(&mut self, integer: i64) {
        self.integer = integer;
    }

    /// Set the 64-bit binary64 fractional remainder.
    pub fn set_fraction(&mut self, fraction: d64) {
        self.fraction = fraction;
    }

    /// Combine integer and fractional components into a single `d64`.
    pub fn to_d64(self) -> d64 {
        (self.integer as d64) + self.fraction
    }
}

/// Construct an `ld128` pair from integer and fractional expressions.
#[macro_export]
macro_rules! ld128 {
    ($int:expr, $frac:expr) => {
        $crate::ld128::new($int, $frac)
    };
}
