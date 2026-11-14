//! Proves the Rust projection matches the C floor's widths.
//!
//! `lib/lang/primitive.h` proves the C half with `_Static_assert`; this is the
//! same arithmetic on the Rust side, so a divergence fails one build instead of
//! drifting silently.

use core::mem::{align_of, size_of};

use rangoon::primitive::{any, bytes, const_any, d64, fnptr};

#[test]
fn d64_is_binary64() {
    assert_eq!(size_of::<d64>(), 8);
    assert_eq!(align_of::<d64>(), 8);
    assert_eq!(d64::MAX, f64::MAX);
}

#[test]
fn bytes_is_the_floor_word() {
    assert_eq!(size_of::<bytes>(), 8);
    assert_eq!(align_of::<bytes>(), 8);
    assert_eq!(size_of::<bytes>(), size_of::<usize>());
}

#[test]
fn untyped_pointers_are_one_word() {
    assert_eq!(size_of::<any>(), 8);
    assert_eq!(size_of::<const_any>(), 8);
    assert_eq!(size_of::<fnptr>(), 8);
}

#[test]
fn fnptr_carries_null_in_the_pointer() {
    let null: fnptr = None;
    assert!(null.is_none());
    assert_eq!(size_of::<fnptr>(), size_of::<any>());
}
