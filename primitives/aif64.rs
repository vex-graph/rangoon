//! DEFINITION: `aif64` is a whole-word 64-bit atomic cell for `if64` (`_Atomic if64` projection).
//! OVERVIEW: `#[repr(transparent)] aif64 { word: AtomicU64 }`.
//! Moves the `(integer, fraction)` pair indivisibly using `if64::to_bits` / `from_bits`.

#![allow(non_camel_case_types)]

use core::mem::{align_of, size_of};
use core::sync::atomic::{AtomicU64, Ordering};
use crate::annotation::overview;
use super::if64::if64;

/// `aif64` — whole-word atomic 64-bit `if64` cell matching `lib/lang/primitive.h`.
#[repr(transparent)]
#[derive(Debug)]
#[overview]
pub struct aif64 {
    word: AtomicU64,
}

const _: () = assert!(size_of::<aif64>() == 8);
const _: () = assert!(align_of::<aif64>() == 8);

impl aif64 {
    /// Zero-initialized atomic `if64`.
    pub const fn zero() -> Self {
        Self { word: AtomicU64::new(0) }
    }

    /// Construct a new atomic `if64` initialized to `value`.
    pub const fn new(value: if64) -> Self {
        Self { word: AtomicU64::new(value.to_bits()) }
    }

    /// Atomically load the whole `if64` pair.
    pub fn load(&self, order: Ordering) -> if64 {
        if64::from_bits(self.word.load(order))
    }

    /// Atomically store the whole `if64` pair.
    pub fn store(&self, value: if64, order: Ordering) {
        self.word.store(value.to_bits(), order);
    }

    /// Atomically swap the whole `if64` pair, returning the previous value.
    pub fn swap(&self, value: if64, order: Ordering) -> if64 {
        if64::from_bits(self.word.swap(value.to_bits(), order))
    }

    /// Atomically compare-and-exchange the packed 64-bit representation of `if64`.
    pub fn compare_exchange(
        &self,
        current: if64,
        new: if64,
        success: Ordering,
        failure: Ordering,
    ) -> Result<if64, if64> {
        self.word
            .compare_exchange(current.to_bits(), new.to_bits(), success, failure)
            .map(if64::from_bits)
            .map_err(if64::from_bits)
    }
}

impl Default for aif64 {
    fn default() -> Self {
        Self::zero()
    }
}
