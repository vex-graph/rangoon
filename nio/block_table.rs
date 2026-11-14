//! DEFINITION: BlockTable is the owner-private generation directory for engine
//! blocks. It is keyed by payload address: the slot keeps the current generation
//! and whether it is live. The table never crosses the seam; C receives only
//! BlockHandle records. Releasing validates the generation before the owner
//! calls the raw Memory_free floor.
//! OVERVIEW: BlockTable { slots: HashMap<usize, BlockSlot, PointerBuildHasher>,
//! generation_limit: u32, live: usize }.
//! API: new/zero/new_with_generation_limit/issue/validate/resolve/release/
//! generation_at/live_count/len/is_empty/to_string/to_string_struct.
//! Generation zero means retired, never a live identity. A released slot keeps
//! its generation so a later issue at the same address advances it and a stale
//! handle rejects. Collision handling is the cost of the pointer key.
use crate::annotation::overview;
use super::block_handle::BlockHandle;
use super::block_slot::BlockSlot;
use super::pointer_hasher::PointerHasher;
use super::storage_error::StorageError;
use super::projection;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;

#[crate::annotation::intention("the pointer is the deliberate key: a block is named by where its payload lives, and the table is owner-private metadata that never crosses the seam")]
const POINTER_KEY_CONTRACT: () = ();

type PointerBuildHasher = BuildHasherDefault<PointerHasher>;

#[overview]
pub struct BlockTable {
    slots: HashMap<usize, BlockSlot, PointerBuildHasher>,  // address -> generation state
    generation_limit: u32,                                 // last issuable generation, immutable
    live: usize,                                           // currently issued handles
}

impl Default for BlockTable {
    /// Create the same empty table as `BlockTable::new`.
    fn default() -> Self { Self::new() }
}

impl BlockTable {
    /// Create an empty table with the default generation budget.
    pub fn new() -> Self { Self::new_with_generation_limit(u32::MAX) }

    /// Create an empty table using the named default generation budget.
    pub fn zero() -> Self { Self::new() }

    /// Choose a smaller immutable generation budget; zero is rejected.
    pub fn new_with_generation_limit(generation_limit: u32) -> Self {
        let () = POINTER_KEY_CONTRACT;
        Self { slots: HashMap::default(), generation_limit, live: 0 }
    }

    /// Register a newly allocated payload address and return its identity.
    /// A live address rejects a second issue; a released address advances.
    pub fn issue(&mut self, pointer: usize) -> Result<BlockHandle, StorageError> {
        if pointer == 0 { return Err(StorageError::Layout); }
        match self.slots.get_mut(&pointer) {
            Some(slot) => {
                if slot.live { return Err(StorageError::Layout); }
                if slot.generation == 0 { return Err(StorageError::Capacity); }
                slot.live = true;
                self.live += 1;
                Ok(BlockHandle::new(pointer, slot.generation))
            }
            None => {
                let slot = BlockSlot { generation: 1, live: true };
                self.slots.insert(pointer, slot);
                self.live += 1;
                Ok(BlockHandle::new(pointer, 1))
            }
        }
    }

    /// Whether the handle names a live incarnation of a known address.
    pub fn validate(&self, handle: BlockHandle) -> bool {
        if handle.pointer() == 0 || handle.generation() == 0 { return false; }
        match self.slots.get(&handle.pointer()) {
            Some(slot) => slot.live && slot.generation == handle.generation(),
            None => false,
        }
    }

    /// Resolve a live handle to its payload address; a stale handle returns `None`.
    pub fn resolve(&self, handle: BlockHandle) -> Option<usize> {
        self.validate(handle).then_some(handle.pointer())
    }

    /// Invalidate one incarnation and advance its generation. The address stays
    /// in the table so a later issue there cannot reissue the stale generation.
    pub fn release(&mut self, handle: BlockHandle) -> Result<(), StorageError> {
        if !self.validate(handle) { return Err(StorageError::Bounds); }
        let slot = self.slots.get_mut(&handle.pointer()).ok_or(StorageError::Bounds)?;
        slot.live = false;
        slot.generation = if slot.generation == self.generation_limit { 0 } else { slot.generation + 1 };
        self.live -= 1;
        Ok(())
    }

    /// Current generation for an address; zero when unknown or retired.
    pub fn generation_at(&self, pointer: usize) -> u32 {
        self.slots.get(&pointer).map_or(0, |slot| slot.generation)
    }

    /// Number of currently issued handles.
    pub fn live_count(&self) -> usize { self.live }

    /// Number of addresses the table has ever seen, live or retired.
    pub fn len(&self) -> usize { self.slots.len() }

    /// Whether the table has never seen an address.
    pub fn is_empty(&self) -> bool { self.slots.is_empty() }

    /// Write a bounded cold value summary.
    pub fn to_string(&self, dest: &mut [u8], truncated: &mut bool) -> bool {
        projection::write(format!("BlockTable(live={}, slots={})", self.live, self.slots.len()), dest, truncated)
    }

    /// Write own fields in declaration order, without walking slots.
    pub fn to_string_struct(&self, dest: &mut [u8], truncated: &mut bool) -> bool {
        projection::write(format!("BlockTable {{ slots: [{} addresses], generation_limit: {}, live: {} }}",
            self.slots.len(), self.generation_limit, self.live), dest, truncated)
    }
}

#[macro_export]
macro_rules! BlockTable {
    () => { $crate::nio::block_table::BlockTable::new() };
    ($generation_limit:expr) => { $crate::nio::block_table::BlockTable::new_with_generation_limit($generation_limit) };
}
