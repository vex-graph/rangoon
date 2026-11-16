//! DEFINITION: `ald128` is two independently atomic halves `{ integer: ai64, fraction: ad64 }`.
//! OVERVIEW: `#[repr(C, align(8))] ald128 { integer: AtomicI64, fraction: ad64 }`.
//! Matches `lib/lang/primitive.h`: not a single 128-bit CAS; each half is independently atomic.

#![allow(non_camel_case_types)]

use core::mem::{align_of, offset_of, size_of};
use core::sync::atomic::{AtomicI64, Ordering};
use crate::annotation::overview;
use crate::primitive::d64;
use super::ad64::ad64;
use super::ld128::ld128;

/// `ald128` — dual-atomic 128-bit `{ ai64 integer; ad64 fraction; }` matching `lib/lang/primitive.h`.
#[repr(C, align(8))]
#[derive(Debug)]
#[overview]
pub struct ald128 {
    integer: AtomicI64,
    fraction: ad64,
}

const _: () = assert!(size_of::<ald128>() == 16);
const _: () = assert!(align_of::<ald128>() == 8);
const _: () = assert!(offset_of!(ald128, integer) == 0);
const _: () = assert!(offset_of!(ald128, fraction) == 8);

impl ald128 {
    /// Zero-initialized `ald128` `{ integer: 0, fraction: 0.0 }`.
    pub const fn zero() -> Self {
        Self { integer: AtomicI64::new(0), fraction: ad64::zero() }
    }

    /// Construct an `ald128` from initial integer and fractional values.
    pub const fn new(integer: i64, fraction: d64) -> Self {
        Self { integer: AtomicI64::new(integer), fraction: ad64::new(fraction) }
    }

    /// Construct an `ald128` from an existing `ld128`.
    pub const fn from_ld128(value: ld128) -> Self {
        Self::new(value.integer(), value.fraction())
    }

    /// Borrow the atomic 64-bit integer half.
    pub const fn integer(&self) -> &AtomicI64 {
        &self.integer
    }

    /// Borrow the atomic 64-bit fractional half.
    pub const fn fraction(&self) -> &ad64 {
        &self.fraction
    }

    /// Independently load both halves using `order` (not a single 128-bit snapshot).
    pub fn load_parts(&self, order: Ordering) -> ld128 {
        ld128::new(self.integer.load(order), self.fraction.load(order))
    }

    /// Independently store both halves using `order` (not a single 128-bit store).
    pub fn store_parts(&self, value: ld128, order: Ordering) {
        self.integer.store(value.integer(), order);
        self.fraction.store(value.fraction(), order);
    }
}

impl Default for ald128 {
    fn default() -> Self {
        Self::zero()
    }
}
