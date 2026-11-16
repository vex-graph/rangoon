//! DEFINITION: `ad64` is an atomic 64-bit binary64 float cell (`_Atomic d64` projection).
//! OVERVIEW: `#[repr(transparent)] ad64 { bits: AtomicU64 }`.
//! Operations move raw IEEE-754 bits atomically with caller-specified `Ordering`.

#![allow(non_camel_case_types)]

use core::mem::{align_of, size_of};
use core::sync::atomic::{AtomicU64, Ordering};
use crate::annotation::overview;
use crate::primitive::d64;

/// `ad64` — atomic 64-bit binary64 float cell matching `lib/lang/primitive.h`.
#[repr(transparent)]
#[derive(Debug)]
#[overview]
pub struct ad64 {
    bits: AtomicU64,
}

const _: () = assert!(size_of::<ad64>() == 8);
const _: () = assert!(align_of::<ad64>() == 8);

impl ad64 {
    /// Zero-initialized atomic `d64` (`+0.0`).
    pub const fn zero() -> Self {
        Self { bits: AtomicU64::new(0) }
    }

    /// Construct a new atomic `d64` initialized to `value`.
    pub const fn new(value: d64) -> Self {
        Self { bits: AtomicU64::new(value.to_bits()) }
    }

    /// Atomically load the current `d64` value.
    pub fn load(&self, order: Ordering) -> d64 {
        d64::from_bits(self.bits.load(order))
    }

    /// Atomically store `value`.
    pub fn store(&self, value: d64, order: Ordering) {
        self.bits.store(value.to_bits(), order);
    }

    /// Atomically swap the stored value with `value`, returning the previous `d64`.
    pub fn swap(&self, value: d64, order: Ordering) -> d64 {
        d64::from_bits(self.bits.swap(value.to_bits(), order))
    }

    /// Atomically compare-and-exchange raw `d64` bit patterns.
    pub fn compare_exchange(
        &self,
        current: d64,
        new: d64,
        success: Ordering,
        failure: Ordering,
    ) -> Result<d64, d64> {
        self.bits
            .compare_exchange(current.to_bits(), new.to_bits(), success, failure)
            .map(d64::from_bits)
            .map_err(d64::from_bits)
    }

    /// Spurious-failure-permitting atomic compare-and-exchange over raw `d64` bit patterns.
    pub fn compare_exchange_weak(
        &self,
        current: d64,
        new: d64,
        success: Ordering,
        failure: Ordering,
    ) -> Result<d64, d64> {
        self.bits
            .compare_exchange_weak(current.to_bits(), new.to_bits(), success, failure)
            .map(d64::from_bits)
            .map_err(d64::from_bits)
    }
}

impl Default for ad64 {
    fn default() -> Self {
        Self::zero()
    }
}
