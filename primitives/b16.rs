//! DEFINITION: `b16` is the 16-bit bfloat16 storage projection of `lib/lang/primitive.h`.
//! OVERVIEW: `#[repr(C)] b16 { bits: u16 }` (8-bit exponent, 7-bit mantissa; NOT IEEE binary16).
//! Arithmetic happens in `f32`; `from_f32` rounds to nearest even and preserves quiet NaNs.

#![allow(non_camel_case_types)]

use core::fmt;
use core::mem::{align_of, size_of};
use crate::annotation::overview;

/// `b16` — 16-bit Brain Floating Point bit pattern matching `lib/lang/primitive.h`.
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
#[overview]
pub struct b16 {
    bits: u16,
}

const _: [(); 2] = [(); size_of::<b16>()];
const _: () = assert!(align_of::<b16>() == 2);

impl b16 {
    /// Zero-initialized `b16` bit pattern (`+0.0`).
    pub const fn zero() -> Self {
        Self { bits: 0 }
    }

    /// Wrap raw 16-bit bfloat16 storage bits.
    pub const fn from_bits(bits: u16) -> Self {
        Self { bits }
    }

    /// Return the underlying 16-bit bfloat16 storage bits.
    pub const fn bits(self) -> u16 {
        self.bits
    }

    /// Overwrite the underlying 16-bit bfloat16 storage bits.
    pub fn set_bits(&mut self, bits: u16) {
        self.bits = bits;
    }

    /// Quantize an `f32` using round-to-nearest-even and quiet-NaN preservation.
    pub fn from_f32(value: f32) -> Self {
        let bits = value.to_bits();
        if (bits & 0x7F80_0000) == 0x7F80_0000 && (bits & 0x007F_FFFF) != 0 {
            return Self { bits: ((bits >> 16) as u16) | 0x0040 };
        }
        let round_bias = 0x0000_7FFF + ((bits >> 16) & 1);
        Self { bits: (bits.wrapping_add(round_bias) >> 16) as u16 }
    }

    /// Widen this `b16` bit pattern to `f32`.
    pub fn to_f32(self) -> f32 {
        f32::from_bits((self.bits as u32) << 16)
    }
}

impl fmt::Debug for b16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("b16")
            .field("bits", &self.bits)
            .field("f32", &self.to_f32())
            .finish()
    }
}

impl fmt::Display for b16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_f32())
    }
}

impl From<f32> for b16 {
    fn from(value: f32) -> Self {
        Self::from_f32(value)
    }
}

impl From<b16> for f32 {
    fn from(value: b16) -> Self {
        value.to_f32()
    }
}

/// Construct a `b16` storage value from an `f32` expression.
#[macro_export]
macro_rules! b16 {
    ($val:expr) => {
        $crate::b16::from_f32($val)
    };
}
