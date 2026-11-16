//! DEFINITION: `if64` is the 64-bit split `{ integer: i32, fraction: f32 }` projection.
//! OVERVIEW: `#[repr(C, align(8))] if64 { integer: i32, fraction: f32 }`.
//! Preserves C ABI size (8), alignment (8), and field offsets (`integer@0`, `fraction@4`).

#![allow(non_camel_case_types)]

use core::mem::{align_of, offset_of, size_of};
use crate::annotation::overview;

/// `if64` — 64-bit integer + fraction coordinate pair matching `lib/lang/primitive.h`.
#[repr(C, align(8))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[overview]
pub struct if64 {
    pub integer: i32,
    pub fraction: f32,
}

const _: () = assert!(size_of::<if64>() == 8);
const _: () = assert!(align_of::<if64>() == 8);
const _: () = assert!(offset_of!(if64, integer) == 0);
const _: () = assert!(offset_of!(if64, fraction) == 4);

impl if64 {
    /// Zero value `{ integer: 0, fraction: 0.0 }`.
    pub const fn zero() -> Self {
        Self { integer: 0, fraction: 0.0 }
    }

    /// Construct an `if64` from integer and fractional components.
    pub const fn new(integer: i32, fraction: f32) -> Self {
        Self { integer, fraction }
    }

    /// Return the signed 32-bit integer component.
    pub const fn integer(self) -> i32 {
        self.integer
    }

    /// Return the 32-bit fractional remainder.
    pub const fn fraction(self) -> f32 {
        self.fraction
    }

    /// Set the signed 32-bit integer component.
    pub fn set_integer(&mut self, integer: i32) {
        self.integer = integer;
    }

    /// Set the 32-bit fractional remainder.
    pub fn set_fraction(&mut self, fraction: f32) {
        self.fraction = fraction;
    }

    /// Combine integer and fractional components into a single `f32`.
    pub fn to_f32(self) -> f32 {
        (self.integer as f32) + self.fraction
    }

    /// Pack the in-memory representation into a 64-bit integer word.
    pub const fn to_bits(self) -> u64 {
        #[cfg(target_endian = "little")]
        {
            (self.integer as u32 as u64) | ((self.fraction.to_bits() as u64) << 32)
        }
        #[cfg(target_endian = "big")]
        {
            ((self.integer as u32 as u64) << 32) | (self.fraction.to_bits() as u64)
        }
    }

    /// Reconstruct an `if64` from its packed 64-bit representation.
    pub const fn from_bits(bits: u64) -> Self {
        #[cfg(target_endian = "little")]
        {
            Self {
                integer: (bits & 0xFFFF_FFFF) as u32 as i32,
                fraction: f32::from_bits((bits >> 32) as u32),
            }
        }
        #[cfg(target_endian = "big")]
        {
            Self {
                integer: (bits >> 32) as u32 as i32,
                fraction: f32::from_bits((bits & 0xFFFF_FFFF) as u32),
            }
        }
    }
}

/// Construct an `if64` pair from integer and fractional expressions.
#[macro_export]
macro_rules! if64 {
    ($int:expr, $frac:expr) => {
        $crate::if64::new($int, $frac)
    };
}
