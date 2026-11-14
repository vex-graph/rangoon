//! The Rust projection of `lib/lang/primitive.h`.
//!
//! The C floor owns the vocabulary; this module only spells it in Rust. Where
//! the token is already the name (`u8`..`u64`, `i8`..`i64`, `f32`, `bool`), no
//! alias exists — `u8` is `u8` in both languages. The aliases below are the gap:
//! names C needs because it has no builtin, and names Rust spells differently.
//!
//! Owned types, not aliases: `b16` is a 2-byte bfloat16 bit pattern, and the
//! atomic spellings `af32`/`ad64`/`ald128` have no `core` type because Rust has
//! no float atomics. They are classes with their own files, not projections.
//!
//! `pointer` is deliberately not aliased: Rust spells a pointer `*mut T` /
//! `*const T` and an address-as-integer `usize`; a third name for `usize`
//! would only hide which one a call site means.

#![allow(non_camel_case_types)]

use core::ffi::c_void;
use core::mem::{align_of, size_of};

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

// The C floor proves these with `_Static_assert`; this is the Rust half of the
// same arithmetic. A divergence fails one side's build, never both silently.
const _: () = assert!(size_of::<d64>() == 8);
const _: () = assert!(align_of::<d64>() == 8);
const _: () = assert!(size_of::<bytes>() == 8);
const _: () = assert!(align_of::<bytes>() == 8);
const _: () = assert!(size_of::<any>() == 8);
const _: () = assert!(size_of::<const_any>() == 8);
const _: () = assert!(size_of::<fnptr>() == 8);
