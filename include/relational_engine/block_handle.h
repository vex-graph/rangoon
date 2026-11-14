#ifndef RE_BLOCK_HANDLE_H
#define RE_BLOCK_HANDLE_H

#include "vex.h"

/* Generation-tagged identity for one incarnation of an engine block, not an
 * address/type ID/serialized record. The address field is an integer, never an
 * untyped pointer: releasing needs a deliberate cast, and the owner's release
 * is the contract. Zero is invalid and never issued. Exact repr(C) Rust peer:
 * nio/block_handle.rs. Only the owning ReBlockTable validates a handle. */
typedef struct ReBlockHandle {
    pointer pointer;    // payload address as an integer, not a live pointer
    u32 generation;     // slot generation captured at issue
} ReBlockHandle;

_Static_assert(sizeof(ReBlockHandle) == 16, "ReBlockHandle must stay two 64-bit words");

#define ReBlockHandle_zero() ((ReBlockHandle) {0})
#endif
