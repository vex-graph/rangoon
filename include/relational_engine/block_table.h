#ifndef RE_BLOCK_TABLE_H
#define RE_BLOCK_TABLE_H

#include "vex.h"
#include "relational_engine/block_handle.h"

/* Opaque owner-private generation directory keyed by payload address. The table
 * is metadata, never ecosystem storage and never crosses the seam; C receives
 * ReBlockHandle records only. The owner issues a handle after it allocates a
 * block, validates the handle before any use, and releases it before calling
 * the raw Memory_free floor — so the generation check never enters Memory_free.
 * A released address keeps its generation, so a later issue there advances it
 * and a stale handle rejects. Status: 0 success, 1 invalid argument,
 * 2 allocation/exhaustion, 3 stale/unknown handle, 4 capacity too small.
 * Owner is exclusive for new/drop/issue/release and shared for the rest.
 * No per-frame guarantee; projections are cold and may allocate. */
typedef struct ReBlockTable ReBlockTable;

ReBlockTable *re_blocks_new();
ReBlockTable *re_blocks_new_with_limit(u32 generation_limit);
fn re_blocks_drop(ReBlockTable *owner);
u32 re_blocks_issue(ReBlockTable *owner, pointer address, ReBlockHandle *output);
u32 re_blocks_validate(const ReBlockTable *owner, ReBlockHandle handle);
u32 re_blocks_resolve(const ReBlockTable *owner, ReBlockHandle handle, pointer *output);
u32 re_blocks_release(ReBlockTable *owner, ReBlockHandle handle);
u32 re_blocks_generation(const ReBlockTable *owner, pointer address, u32 *output);
u32 re_blocks_live(const ReBlockTable *owner, bytes *output);
u32 re_blocks_to_string(const ReBlockTable *owner, u8 *dest, bytes capacity, bool *truncated);
u32 re_blocks_to_string_struct(const ReBlockTable *owner, u8 *dest, bytes capacity, bool *truncated);

#endif
