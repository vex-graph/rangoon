//! Proves the Rust projection matches the C floor's widths.
//!
//! `lib/lang/primitive.h` proves the C half with `_Static_assert`; this is the
//! same arithmetic on the Rust side, so a divergence fails one build instead of
//! drifting silently.

use core::mem::{align_of, offset_of, size_of};
use core::sync::atomic::Ordering;

use rangoon::primitive::{
    ad64, af32, ai16, ai32, ai64, ai8, aif64, ald128, any, au16, au32, au64, au8, bytes,
    const_any, d64, fnptr,
};
use rangoon::{b16, if64, ld128};

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

#[test]
fn atomic_integers_match_scalar_widths() {
    assert_eq!((size_of::<ai8>(), align_of::<ai8>()), (1, 1));
    assert_eq!((size_of::<ai16>(), align_of::<ai16>()), (2, 2));
    assert_eq!((size_of::<ai32>(), align_of::<ai32>()), (4, 4));
    assert_eq!((size_of::<ai64>(), align_of::<ai64>()), (8, 8));
    assert_eq!((size_of::<au8>(), align_of::<au8>()), (1, 1));
    assert_eq!((size_of::<au16>(), align_of::<au16>()), (2, 2));
    assert_eq!((size_of::<au32>(), align_of::<au32>()), (4, 4));
    assert_eq!((size_of::<au64>(), align_of::<au64>()), (8, 8));

    let signed = ai32::new(-7);
    assert_eq!(signed.swap(19, Ordering::SeqCst), -7);
    assert_eq!(signed.load(Ordering::SeqCst), 19);

    let unsigned = au64::new(42);
    assert_eq!(unsigned.compare_exchange(42, 99, Ordering::AcqRel, Ordering::Acquire), Ok(42));
    assert_eq!(unsigned.load(Ordering::Acquire), 99);
}

#[test]
fn b16_round_trip_rounding_and_nan_preservation() {
    assert_eq!(size_of::<b16>(), 2);
    assert_eq!(align_of::<b16>(), 2);
    assert_eq!(b16::zero().bits(), 0);

    let half = b16!(1.5_f32);
    assert_eq!(half.bits(), 0x3FC0);
    assert_eq!(half.to_f32().to_bits(), 1.5_f32.to_bits());

    // Round-to-nearest-even tie case:
    // LSB of upper 16 bits is 0 (`0x3F80_8000`) -> stays `0x3F80` (round to even).
    assert_eq!(b16::from_f32(f32::from_bits(0x3F80_8000)).bits(), 0x3F80);
    // LSB of upper 16 bits is 1 (`0x3F81_8000`) -> rounds up to `0x3F82` (even).
    assert_eq!(b16::from_f32(f32::from_bits(0x3F81_8000)).bits(), 0x3F82);

    // Signaling NaN becomes quiet NaN (bit 0x0040 preserved) and never turns into Inf.
    let snan = f32::from_bits(0x7F80_0001);
    let q = b16::from_f32(snan);
    assert_ne!(q.bits() & 0x007F, 0);
    assert!(q.to_f32().is_nan());

    let mut cell = b16::from_bits(0);
    cell.set_bits(0x4000);
    assert_eq!(f32::from(cell).to_bits(), 2.0_f32.to_bits());
}

#[test]
fn if64_and_ld128_layout_and_conversion() {
    assert_eq!((size_of::<if64>(), align_of::<if64>()), (8, 8));
    assert_eq!(offset_of!(if64, integer), 0);
    assert_eq!(offset_of!(if64, fraction), 4);

    assert_eq!((size_of::<ld128>(), align_of::<ld128>()), (16, 8));
    assert_eq!(offset_of!(ld128, integer), 0);
    assert_eq!(offset_of!(ld128, fraction), 8);

    let mut small = if64!(12, 0.25_f32);
    assert_eq!(small.integer(), 12);
    assert_eq!(small.fraction().to_bits(), 0.25_f32.to_bits());
    assert_eq!(small.to_f32().to_bits(), 12.25_f32.to_bits());
    small.set_integer(-3);
    small.set_fraction(-0.5_f32);
    assert_eq!(small.to_f32().to_bits(), (-3.5_f32).to_bits());
    assert_eq!(if64::from_bits(small.to_bits()), small);

    let mut wide = ld128!(1_000_000_000_000_i64, 0.125_f64);
    assert_eq!(wide.integer(), 1_000_000_000_000_i64);
    assert_eq!(wide.fraction().to_bits(), 0.125_f64.to_bits());
    assert!((wide.to_d64() - 1_000_000_000_000.125_f64).abs() < 1e-9);
    wide.set_integer(4);
    wide.set_fraction(0.5);
    assert_eq!(wide.to_d64().to_bits(), 4.5_f64.to_bits());
}

#[test]
fn float_and_composite_atomics() {
    assert_eq!((size_of::<af32>(), align_of::<af32>()), (4, 4));
    let f = af32::new(1.5);
    assert_eq!(f.load(Ordering::Acquire).to_bits(), 1.5_f32.to_bits());
    assert_eq!(f.swap(2.5, Ordering::AcqRel).to_bits(), 1.5_f32.to_bits());
    assert_eq!(
        f.compare_exchange(2.5, 3.25, Ordering::SeqCst, Ordering::Acquire).map(f32::to_bits),
        Ok(2.5_f32.to_bits()),
    );
    f.store(0.0, Ordering::Release);
    assert_eq!(f.load(Ordering::Acquire).to_bits(), 0.0_f32.to_bits());

    assert_eq!((size_of::<ad64>(), align_of::<ad64>()), (8, 8));
    let d = ad64::new(6.25);
    assert_eq!(d.swap(-1.25, Ordering::AcqRel).to_bits(), 6.25_f64.to_bits());
    assert_eq!(
        d.compare_exchange(-1.25, 8.5, Ordering::SeqCst, Ordering::Acquire).map(f64::to_bits),
        Ok((-1.25_f64).to_bits()),
    );
    assert_eq!(d.load(Ordering::Acquire).to_bits(), 8.5_f64.to_bits());

    assert_eq!((size_of::<aif64>(), align_of::<aif64>()), (8, 8));
    let a = aif64::new(if64!(7, 0.25));
    let prev = a.swap(if64!(-9, -0.5), Ordering::AcqRel);
    assert_eq!(prev, if64!(7, 0.25));
    assert_eq!(
        a.compare_exchange(if64!(-9, -0.5), if64!(10, 0.75), Ordering::SeqCst, Ordering::Acquire),
        Ok(if64!(-9, -0.5)),
    );
    assert_eq!(a.load(Ordering::Acquire), if64!(10, 0.75));

    assert_eq!((size_of::<ald128>(), align_of::<ald128>()), (16, 8));
    let al = ald128::from_ld128(ld128!(55, 0.5));
    assert_eq!(al.integer().load(Ordering::Acquire), 55);
    assert_eq!(al.fraction().load(Ordering::Acquire).to_bits(), 0.5_f64.to_bits());
    al.store_parts(ld128!(-12, -0.25), Ordering::Release);
    assert_eq!(al.load_parts(Ordering::Acquire), ld128!(-12, -0.25));
}
