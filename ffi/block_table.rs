//! DEFINITION: procedural C adapter for the Rust BlockTable; declarations in
//! block_table.h. The table is owner-private metadata keyed by payload address;
//! C receives BlockHandle records only. The owner validates a handle before it
//! calls the raw Memory_free floor, so the generation check never enters Memory_free.
//! OVERVIEW: new/new_with_limit/drop/issue/validate/resolve/release/generation/
//! live and bounded projections, all re_blocks_* exports.
//! Integer status is observable rejection (same Rust FFI policy as memory/rows):
//! 0 success, 1 invalid argument, 2 allocation/exhaustion, 3 stale/unknown handle.
//! Pointer arguments must be live and externally synchronized; the table does
//! not validate a raw address, only the generation a handle carries.
//! No unwind recovery; projections are cold and may allocate.
use crate::nio::block_handle::BlockHandle;
use crate::nio::block_table::BlockTable;
use crate::nio::storage_error::StorageError;

/// Map storage failures to fixed public ABI codes.
fn status(error: StorageError) -> u32 {
    match error {
        StorageError::Allocation | StorageError::Capacity => 2,
        StorageError::Bounds => 3,
        _ => 1,
    }
}

/// Allocate a new opaque generation table for the C ABI.
#[unsafe(no_mangle)]
pub extern "C" fn re_blocks_new() -> *mut BlockTable {
    Box::into_raw(Box::new(BlockTable::new()))
}

/// Allocate a table with an explicit non-wrapping generation budget.
#[unsafe(no_mangle)]
pub extern "C" fn re_blocks_new_with_limit(generation_limit: u32) -> *mut BlockTable {
    Box::into_raw(Box::new(BlockTable::new_with_generation_limit(generation_limit)))
}

/// # Safety
/// Owner is null or a live result of re_blocks_new, released exactly once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn re_blocks_drop(owner: *mut BlockTable) {
    if !owner.is_null() { drop(unsafe { Box::from_raw(owner) }); }
}

/// # Safety
/// Owner is live and exclusive; output points to writable BlockHandle storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn re_blocks_issue(owner: *mut BlockTable, pointer: usize, output: *mut BlockHandle) -> u32 {
    if owner.is_null() || output.is_null() { return 1; }
    match unsafe { &mut *owner }.issue(pointer) {
        Ok(handle) => { unsafe { *output = handle }; 0 }
        Err(error) => status(error),
    }
}

/// # Safety
/// Owner is live and shared without concurrent mutation.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn re_blocks_validate(owner: *const BlockTable, handle: BlockHandle) -> u32 {
    if owner.is_null() { return 1; }
    if unsafe { &*owner }.validate(handle) { 0 } else { 3 }
}

/// # Safety
/// Owner is live and shared; output points to writable usize storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn re_blocks_resolve(owner: *const BlockTable, handle: BlockHandle, output: *mut usize) -> u32 {
    if owner.is_null() || output.is_null() { return 1; }
    match unsafe { &*owner }.resolve(handle) {
        Some(pointer) => { unsafe { *output = pointer }; 0 }
        None => 3,
    }
}

/// # Safety
/// Owner is live and exclusive; the caller has stopped using the released block.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn re_blocks_release(owner: *mut BlockTable, handle: BlockHandle) -> u32 {
    if owner.is_null() { return 1; }
    match unsafe { &mut *owner }.release(handle) {
        Ok(()) => 0,
        Err(error) => status(error),
    }
}

/// # Safety
/// Owner is live and shared; output points to writable u32 storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn re_blocks_generation(owner: *const BlockTable, pointer: usize, output: *mut u32) -> u32 {
    if owner.is_null() || output.is_null() { return 1; }
    unsafe { *output = (&*owner).generation_at(pointer); }
    0
}

/// # Safety
/// Owner is live and shared; output points to writable usize storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn re_blocks_live(owner: *const BlockTable, output: *mut usize) -> u32 {
    if owner.is_null() || output.is_null() { return 1; }
    unsafe { *output = (&*owner).live_count(); }
    0
}

/// Cold projection adapter; null owner emits nullptr, zero capacity is flagged truncation.
unsafe fn project(owner: *const BlockTable, dest: *mut u8, capacity: usize, truncated: *mut bool, structure: bool) -> u32 {
    if truncated.is_null() || capacity > isize::MAX as usize || (dest.is_null() && capacity != 0) { return 1; }
    let dest = if capacity == 0 { &mut [] } else { unsafe { std::slice::from_raw_parts_mut(dest, capacity) } };
    let flag = unsafe { &mut *truncated };
    let complete = if owner.is_null() {
        crate::nio::projection::write("nullptr".to_owned(), dest, flag)
    } else if structure {
        unsafe { &*owner }.to_string_struct(dest, flag)
    } else {
        unsafe { &*owner }.to_string(dest, flag)
    };
    if complete { 0 } else { 4 }
}

/// # Safety
/// Owner null or live/shared; dest and flag writable/disjoint. Formatting is cold only.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn re_blocks_to_string(owner: *const BlockTable, dest: *mut u8, capacity: usize, truncated: *mut bool) -> u32 {
    unsafe { project(owner, dest, capacity, truncated, false) }
}

/// # Safety
/// Same pointer/exclusion contract as re_blocks_to_string; renders one layer only.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn re_blocks_to_string_struct(owner: *const BlockTable, dest: *mut u8, capacity: usize, truncated: *mut bool) -> u32 {
    unsafe { project(owner, dest, capacity, truncated, true) }
}
