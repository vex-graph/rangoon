//! DEFINITION: BlockHandle is a generation-tagged identity for one incarnation
//! of an engine block. The address field is an integer, never an untyped
//! pointer: releasing needs a deliberate cast, and the owner's release is the
//! contract. Zero is invalid and never issued.
//! OVERVIEW: repr(C) BlockHandle { pointer: usize, generation: u32 }.
//! API: new/zero/pointer/generation/is_zero/to_string/to_string_struct.
//! The handle names a block; it owns nothing and borrows nothing. Only the
//! owner's BlockTable can validate it, and a stale generation rejects.
use crate::annotation::overview;
use super::projection;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[overview]
pub struct BlockHandle {
    pointer: usize,   // payload address as an integer, never an untyped pointer
    generation: u32,  // slot generation captured at issue; distinguishes a later reuse
}

impl BlockHandle {
    /// Build a candidate identity; only its owning BlockTable can validate it.
    pub fn new(pointer: usize, generation: u32) -> Self {
        Self { pointer, generation }
    }

    /// Return the invalid zero identity.
    pub fn zero() -> Self { Self::new(0, 0) }

    /// Return the payload address as an integer, not a live pointer.
    pub fn pointer(&self) -> usize { self.pointer }

    /// Return the generation captured at issue.
    pub fn generation(&self) -> u32 { self.generation }

    /// Whether this handle can never be valid.
    pub fn is_zero(&self) -> bool { self.pointer == 0 || self.generation == 0 }

    /// Write a bounded human value projection (cold, may allocate).
    pub fn to_string(&self, dest: &mut [u8], truncated: &mut bool) -> bool {
        projection::write(format!("BlockHandle(0x{:x}, {})", self.pointer, self.generation), dest, truncated)
    }

    /// Write own fields in declaration order (cold, may allocate).
    pub fn to_string_struct(&self, dest: &mut [u8], truncated: &mut bool) -> bool {
        projection::write(format!("BlockHandle {{ pointer: 0x{:x}, generation: {} }}",
            self.pointer, self.generation), dest, truncated)
    }
}

#[macro_export]
macro_rules! BlockHandle {
    () => { $crate::nio::block_handle::BlockHandle::zero() };
    ($pointer:expr, $generation:expr) => {
        $crate::nio::block_handle::BlockHandle::new($pointer, $generation)
    };
}
