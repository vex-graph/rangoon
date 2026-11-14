//! Fast multiply-shift mixer for aligned pointer keys. The default SipHash is
//! general-purpose; addresses are already unique, so one multiply spreads their
//! high bits across the map's index. It is a mixer, not a checksum.
use std::hash::Hasher;

#[derive(Default)]
pub(super) struct PointerHasher(u64);

impl Hasher for PointerHasher {
    fn finish(&self) -> u64 { self.0 }

    fn write(&mut self, bytes: &[u8]) {
        let mut hash = self.0;
        for byte in bytes {
            hash = (hash ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3);
        }
        self.0 = hash;
    }

    fn write_usize(&mut self, value: usize) {
        self.0 = (value as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
    }
}
