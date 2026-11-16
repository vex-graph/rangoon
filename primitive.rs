//! The Rust projection of `lib/lang/primitive.h`.
//!
//! The C floor owns the vocabulary; this module only spells it in Rust. Where
//! the token is already the name (`u8`..`u64`, `i8`..`i64`, `f32`, `bool`), no
//! alias exists — `u8` is `u8` in both languages. The aliases below are the gap:
//! names C needs because it has no builtin, and names Rust spells differently.
//!
//! Owned types, not aliases: `b16`, `if64`, `ld128`, and the atomic cells
//! (`af32`, `ad64`, `aif64`, `ald128`) each live in their own file under
//! `primitives/` (One Rust Class Per File Law) and are re-exported here.
//!
//! `pointer` is deliberately not aliased: Rust spells a pointer `*mut T` /
//! `*const T` and an address-as-integer `usize`; a third name for `usize`
//! would only hide which one a call site means.

#![allow(non_camel_case_types)]

use core::ffi::c_void;
use core::mem::{align_of, size_of};
use core::sync::atomic::{
    AtomicI16, AtomicI32, AtomicI64, AtomicI8,
    AtomicU16, AtomicU32, AtomicU64, AtomicU8,
};

pub use crate::primitives::{
    ad64::ad64, af32::af32, aif64::aif64, ald128::ald128, b16::b16, if64::if64, ld128::ld128,
};

/// `d64` — binary64, the C floor's `double`.
pub type d64 = f64;

/// `bytes` — the type `sizeof` yields, `size_t` on the six-target floor.
pub type bytes = usize;

/// `any` — the one spelling of an untyped pointer (`void*`).
pub type any = *mut c_void;

/// `const_any` — an untyped pointer whose bytes are not written (`const void*`).
pub type const_any = *const c_void;

/// `fnptr` — an untyped function pointer (`any (*)(any)`).
///
/// `Option` is the null: a null function pointer is representable, and
/// `Option<fn>` has the same layout as the bare pointer. Calling it is
/// `unsafe`, which C's spelling does not encode.
pub type fnptr = Option<unsafe extern "C" fn(any) -> any>;

/// Atomic signed integers matching `ai8`..`ai64` in `lib/lang/primitive.h`.
pub type ai8 = AtomicI8;
pub type ai16 = AtomicI16;
pub type ai32 = AtomicI32;
pub type ai64 = AtomicI64;

/// Atomic unsigned integers matching `au8`..`au64` in `lib/lang/primitive.h`.
pub type au8 = AtomicU8;
pub type au16 = AtomicU16;
pub type au32 = AtomicU32;
pub type au64 = AtomicU64;

// The C floor proves these with `_Static_assert`; this is the Rust half of the
// same arithmetic. A divergence fails one side's build, never both silently.
const _: () = assert!(size_of::<d64>() == 8);
const _: () = assert!(align_of::<d64>() == 8);
const _: () = assert!(size_of::<bytes>() == 8);
const _: () = assert!(align_of::<bytes>() == 8);
const _: () = assert!(size_of::<any>() == 8);
const _: () = assert!(size_of::<const_any>() == 8);
const _: () = assert!(size_of::<fnptr>() == 8);
const _: () = assert!(size_of::<ai8>() == 1 && align_of::<ai8>() == 1);
const _: () = assert!(size_of::<ai16>() == 2 && align_of::<ai16>() == 2);
const _: () = assert!(size_of::<ai32>() == 4 && align_of::<ai32>() == 4);
const _: () = assert!(size_of::<ai64>() == 8 && align_of::<ai64>() == 8);
const _: () = assert!(size_of::<au8>() == 1 && align_of::<au8>() == 1);
const _: () = assert!(size_of::<au16>() == 2 && align_of::<au16>() == 2);
const _: () = assert!(size_of::<au32>() == 4 && align_of::<au32>() == 4);
const _: () = assert!(size_of::<au64>() == 8 && align_of::<au64>() == 8);
