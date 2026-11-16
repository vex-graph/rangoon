//! DEFINITION: `af32` is an atomic 32-bit binary32 float cell (`_Atomic f32` projection).
//! OVERVIEW: `#[repr(transparent)] af32 { bits: AtomicU32 }`.
//! Operations move raw IEEE-754 bits atomically with caller-specified `Ordering`.

#![allow(non_camel_case_types)]

use core::mem::{align_of, size_of};
use core::sync::atomic::{AtomicU32, Ordering};
use crate::annotation::overview;

/// `af32` — atomic 32-bit binary32 float cell matching `lib/lang/primitive.h`.
#[repr(transparent)]
#[derive(Debug)]
#[overview]
pub struct af32 {
    bits: AtomicU32,
}

const _: () = assert!(size_of::<af32>() == 4);
const _: () = assert!(align_of::<af32>() == 4);

impl af32 {
    /// Zero-initialized atomic `f32` (`+0.0`).
    pub const fn zero() -> Self {
        Self { bits: AtomicU32::new(0) }
    }

    /// Construct a new atomic `f32` initialized to `value`.
    pub const fn new(value: f32) -> Self {
        Self { bits: AtomicU32::new(value.to_bits()) }
    }

    /// Atomically load the current `f32` value.
    pub fn load(&self, order: Ordering) -> f32 {
        f32::from_bits(self.bits.load(order))
    }

    /// Atomically store `value`.
    pub fn store(&self, value: f32, order: Ordering) {
        self.bits.store(value.to_bits(), order);
    }

    /// Atomically swap the stored value with `value`, returning the previous `f32`.
    pub fn swap(&self, value: f32, order: Ordering) -> f32 {
        f32::from_bits(self.bits.swap(value.to_bits(), order))
    }

    /// Atomically compare-and-exchange raw `f32` bit patterns.
    pub fn compare_exchange(
        &self,
        current: f32,
        new: f32,
        success: Ordering,
        failure: Ordering,
    ) -> Result<f32, f32> {
        self.bits
            .compare_exchange(current.to_bits(), new.to_bits(), success, failure)
            .map(f32::from_bits)
            .map_err(f32::from_bits)
    }

    /// Spurious-failure-permitting atomic compare-and-exchange over raw `f32` bit patterns.
    pub fn compare_exchange_weak(
        &self,
        current: f32,
        new: f32,
        success: Ordering,
        failure: Ordering,
    ) -> Result<f32, f32> {
        self.bits
            .compare_exchange_weak(current.to_bits(), new.to_bits(), success, failure)
            .map(f32::from_bits)
            .map_err(f32::from_bits)
    }
}

impl Default for af32 {
    fn default() -> Self {
        Self::zero()
    }
}
