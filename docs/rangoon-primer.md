# rangoon — the Rust storage surface, plainly

This is a reading document, not a spec. It explains what `rangoon` owns after the
Rust move, how its storage types relate, and how C reaches them through one thin
ABI. It is written so you can follow the Rust half without reading every file, and
so you can disagree with the architecture in an informed way.

If something here contradicts the source, the source wins — this document is a
map, not the territory. The C half has its own primer:
`../../vex/re/docs/relational-engine-primer.md`.

## Current State

`rangoon` is the **Rust vocabulary and storage surface** of the ecosystem. The C
floor is `lib` (`vex.h`, `lang/primitive.h`); the relational engine (`vex/re`)
owns the C allocator. This crate owns typed storage, byte/string values, stable
rows, variable bindings, file mappings and physical preallocation, and ships its
own C ABI.

Implemented and proven on macOS arm64 in debug and release: the vocabulary
projection, `Memory`, `Chunk`/`ChunkedList`, `TypedChunk`/`TypedPool`,
`RowPool`, `VariableSlot`/`VariableRegistry`, `FixedMappedFile`,
`GrowableMappedFile` and `PreallocatedFile`. The owner suite also compiles real C
clients against `include/relational_engine/` and runs C-side ASan/UBSan. Rust ASan
currently skips for a missing compatible runtime; Windows, macOS 14 runtime
compatibility and performance remain unproved.

## 1. The one-sentence version

`rangoon` is the ecosystem's **Rust storage owner**: it holds typed rows and byte
values with explicit lifetimes, keeps their addresses stable, and exposes exactly
the C operations a consumer needs without converting that consumer to Rust.

It is not a second general-purpose allocator. It is not a replacement for the
native `Memory_*` ABI. It does not own windows, GPUs, UI or database policy.

## 2. Where it sits

The dependency arrow points one way:

```
lib (C floor)  <--  re (C allocator + C ABI)  <--  rangoon (Rust)  <--  consumers
```

`re` does not link `rangoon`. `rangoon` does not re-implement the native
allocator. A consumer that wants Rust storage includes
`include/relational_engine/` and links `librangoon.a`; a consumer that wants the
native arena keeps using `re`.

The repository root is `rust/` in the workspace; the crate name is `rangoon` and
the C ABI keeps the `relational_engine/` namespace and the `re_*` symbols.

## 3. The vocabulary projection

`primitive.rs` projects the floor vocabulary into Rust tokens. A token that is
already the name (`u8`..`u64`, `i8`..`i64`, `f32`, `bool`) gets no alias; the gap
is projected once:

- `d64` = `f64`, `bytes` = `usize`
- `any` = `*mut c_void`, `const_any` = `*const c_void`
- `fnptr` = `Option<unsafe extern "C" fn(any) -> any>`

`b16` and the float atomics (`af32`, `ad64`, `ald128`) are owned types, not
aliases: Rust core has no bfloat16 and no float atomics. `pointer` is not
aliased: Rust spells a pointer `*mut T`/`*const T` and an address `usize`.

`fn` is `()` in Rust and `discard` is `let _ = value;` or `drop(value)`, so
neither needs an alias. Compile-time width assertions mirror the C floor's
`_Static_assert`s, so the two halves prove the same arithmetic independently.

## 4. The storage pieces

### 4.1 `Memory` — byte blocks

`nio/mem.rs` owns heap-backed byte blocks. You copy bytes in, get an owner-local
id back, and read them out. Strings can be atomic: a set publishes a whole
immutable byte snapshot with release/acquire, so readers never lock. Old
snapshots stay until owner destruction; the caller chooses the retention budget.

`Memory` is the Rust byte store, not the native `Memory_*` ABI. Its ids are
owner-local and its pointers are its own.

### 4.2 Chunks and pools — stable rows

- **`Chunk<T>`** — a fixed-capacity block of typed rows that never moves.
- **`ChunkedList<T>`** — a growable directory of chunks; initialized rows keep
  their addresses across growth, until owner destruction.
- **`TypedChunk<T>`** — a chunk plus a packed occupancy bitmap, so freed slots
  are reused without moving survivors.
- **`TypedPool<T>`** — a growable directory of `TypedChunk`s.

The point of all four is **stable addresses**. Rows do not shuffle when the
collection grows.

### 4.3 `Handle` — identity, not just location

A raw index is a *location*. Free the slot, reuse it, and an old index silently
points at a different object. `Handle` carries identity:

```
Handle { index: usize, generation: u32 }
```

Every slot starts at generation one. Removal advances it; removal at the maximum
generation retires the slot permanently instead of wrapping. Empty backing
release keeps generation metadata, so recreation cannot resurrect a removed
identity. Rust `Handle` is still owner-local: keep it with its original
`TypedChunk` or `TypedPool`.

### 4.4 `RowPool` — aligned byte rows with C identity

`struct/row_pool.rs` stores fixed-size **bytes**, not Rust objects. Each pool
chooses its row size, power-of-two alignment and rows per chunk. Aligned row
allocations stay put while the directory grows.

`RowChunk` is the per-chunk storage; `RowHandle` is the C-facing identity:

```
RowHandle { owner: u64, index: bytes, generation: u32, reserved: u32 }
```

The process-local owner id rejects a handle from another pool, even after
destruction and recreation. Zero is invalid. Handles never wrap; exhausted slots
retire. This is not forgery protection or a persistent id.

Add copies input; read copies output; write replaces exactly one row; borrow
exposes read-only bytes. Embedded pointers and destructors stay the C caller's
responsibility. All calls require external serialization.

### 4.5 `VariableSlot` and `VariableRegistry` — name to value

A `VariableSlot` is a 32-byte record:

```
{ u8 name[24], pointer-to-value }
```

The name is strictly bounded to 23 ASCII bytes plus a NUL; the pointer addresses
the value's bytes, not the slot's own address. `VariableRegistry` holds slots in a
`ChunkedList` and resolves names through the native C `re_name_search`
primitive.

That primitive lives here, in `search/primitives/name_search.c`, because it scans
Rust-owned rows. `build.rs` compiles it into `librangoon_name_search.a` and links
it into the crate.

### 4.6 The identity model, in one place

Two structures, two jobs:

- **The block header** — `{ id, hash, length, pointer }` — describes the bytes.
  `hash` is a checksum, not an identity: reuse at the same address with the same
  type and length produces the same hash, so it cannot tell an incarnation from
  its successor.
- **The handle** — `{ pointer, generation }` — names one incarnation of a block.
  The address field is `pointer` (`uintptr_t`), never `any`, so `free()` on it
  does not compile; releasing is the owner's `remove`, which is the contract.

The owner holds the table, keyed by the pointer:

```
handle → pointer → table → generation → compare → address
```

The slot stores `{ generation, live }`. The owner validates the generation
**before** it calls `Memory_free(pointer)`. `Memory_free` stays the raw, unchecked
floor; the owner is the checked API, above it — so `re` never calls into Rust.

A pointer-keyed table needs collision handling; a flat index array would not.
That is the cost of using the address as the key, and it is a simplicity trade,
not a safety one.

The invariants:

1. The handle's address field is `pointer` (`uintptr_t`), never `any`; `free()`
   requires a deliberate cast, which the contract forbids.
2. The owner validates the generation before it resolves or releases anything.
3. A stale handle is rejected, never dereferenced.
4. Generation advances on removal; at maximum, the slot retires.
5. Payloads live in engine memory; the table lives with the owner.
6. The owner is destroyed once; table lifetime and arena lifetime are explicit.
7. `Memory_free` is the raw floor; the checked path is the owner.

## 5. The C ABI

`include/relational_engine/relational_engine.h` is the one door. It includes the
surface's headers; a C consumer includes the door, never an individual header.

The rules:

1. **Opaque owners.** C holds `ReMemory`, `ReRowPool`, `ReVariableRegistry`,
   `ReFixedMappedFile` or `ReGrowableMappedFile` as an opaque pointer it never
   dereferences. The Rust side owns the data.
2. **`extern "C"` plus `#[unsafe(no_mangle)]`.** A Rust function marked this way
   has a fixed C name and calling convention.
3. **`#[repr(C)]` for shared records.** `ReRowHandle` and `ReVariableSlot` have an
   exact Rust peer. Scalars and opaque pointers cross; `String`, `Vec` and
   data-carrying enums do not.
4. **Status codes, not exceptions.** Rust panics must never unwind into C. Every
   fallible operation returns a small integer status and writes results through
   caller-supplied outputs. Failures preserve outputs unless a documented
   truncation flag says otherwise.
5. **Destinations last.** Data flows out through pointers the caller supplied;
   the destination and its flags come last.
6. **Explicit ownership and aliasing.** Follow each function's pointer and
   exclusion contract. Creation, copying and string replacement can allocate. C
   returns a Rust-owned owner to its Rust destructor, never to C `free`.
7. **Arbitrary pointers are not validated.** A non-null pointer must be live and
   aligned. The engine trusts the pointer you hand it, exactly as C does.

Status meanings are per surface, not one global enum:

- `Memory`: `0` OK, `1` invalid, `2` allocation/exhaustion, `3` unknown id,
  `4` capacity; atomic extensions `5` wrong kind, `6` byte index bounds,
  `7` writer busy.
- `RowPool`: `0` OK, `1` invalid geometry/span/null, `2` allocation/exhaustion,
  `3` stale/wrong-owner/missing handle, `4` insufficient capacity.
- `FixedMappedFile`: `0` OK, `1` invalid argument, `2` IO/OS failure, `3`
  bounds/overflow, `4` read-only, `5` closed, `6` length unrepresentable or not a
  regular file, `7` destination too small.
- `GrowableMappedFile`: the fixed codes plus `8`, this host has no reservation
  backend.

A concrete example, from `include/relational_engine/memory.h`:

```c
typedef struct ReMemory ReMemory;
ReMemory *re_memory_new();
fn re_memory_drop(ReMemory *owner);
u32 re_memory_copy(ReMemory *owner, const u8 *source, bytes length, u64 *output);
u32 re_memory_read(const ReMemory *owner, u64 id, u8 *destination, bytes capacity,
                   bytes *output_length);
```

`re_memory_new` may abort on allocator OOM; there is no general panic-to-status
catcher. `extern "C"` is not exception recovery.

## 6. Mappings and preallocation

- **`FixedMappedFile`** — a fixed-extent, file-backed byte view through `memmap2`.
  The mapping length is the file's length at open and never changes. Offset/count
  access, exclusive writes, explicit flush and sync, and a read-only borrow that
  dies with the owner. No resize, no mapped C ABI, no implicit flush.
- **`GrowableMappedFile`** — a reservation-backed view. The reservation is mapped
  once and the committed length grows inside it, so a pointer taken before a
  growth step is the same pointer after it. That stability is the whole contract.
  No shrink, truncate, growing reservation or concurrent writers.
- **`PreallocatedFile`** — creates a new physically reserved byte file without a
  sparse fallback and transfers the retained descriptor into `FixedMappedFile`.
  APFS copy-on-write can still require more space than the reservation suggests.

**Unsafe admission is the caller's precondition, not checked for you.** Opening a
mutable file-backed mapping is a trust decision: the caller must exclude external
mutation and truncation of the file, and any other writable view of it, for the
owner's whole lifetime. Rust borrowing cannot constrain another process.

Page faults, open, flush, sync and cleanup have no wall-time bound and stay off
realtime and bounded-join teardown paths. A mapping is not a transaction and does
not provide crash durability.

## 7. Building it

The crate is `rangoon`, edition 2024, `crate-type = ["rlib", "staticlib"]`.
`build.rs` compiles only the engine-owned C search primitive:

```
clang -std=c23 -Wall -Wextra -Werror -O2 -I. -I../lib -c search/primitives/name_search.c
```

It links `librangoon_name_search.a` into the crate. There is no dependency
download. A cross build requires an explicit `CC` and `AR`; MSVC is refused rather
than silently built.

From the crate root:

```sh
python3 -B test/run.py
```

The runner covers the debug/release Rust owners, the C ABI clients, the C-side
ASan/UBSan pass and the compile-negative cases.

## 8. Why Rust is here

The same storage could be written in C. Rust is a tradeoff: more compiler help
inside the storage owner, in exchange for a second language and a maintained ABI.

- **Ownership and borrowing** catch use-after-free, double-free and aliasing bugs
  in safe Rust before the program runs, provided unsafe implementations uphold
  their contracts.
- **`MaybeUninit`, explicit `unsafe` and `Drop`** let the engine build manual,
  bit-level storage while keeping the unsafe surface small and reviewable.
- **Atomics are first-class**, as they are in C. Correct publication still needs
  design and concurrency tests.
- **Type safety at the boundary**: the compiler proves the Rust side's invariants;
  the unsafe bridge remains manually reviewed and tested.

Costs include a second toolchain, explicit unsafe code, header drift, and
restrictions on aliasing. Rust does not automatically make a path allocation-free
or realtime-safe: `Vec`, `Box` and `format!` still allocate.

## 9. What is real vs planned

**Real:** the vocabulary, `Memory`, stable chunks and typed pools, `RowPool` and
its C ABI, `VariableSlot`/`VariableRegistry` with native name search, the fixed
and growable mappings, physical preallocation, and real C-client proof on macOS
arm64.

**Not proved:** Windows execution, macOS 14 runtime compatibility of the installed
Rust standard library, Rust sanitizer instrumentation, measured performance, full
fault/concurrency coverage, and downstream consumer migration. No C/Rust
atomic-layout compatibility is assumed.

**Not offered:** a database, transactions, crash durability, a file format,
automatic schema migration, live reload integration, or a second general-purpose
allocator.

## 10. The distinctions to keep in your head

- **Address:** where bytes are now. Stable across growth is not immortal.
- **Identity:** which object you mean, even when storage locations are reused.
- **Name:** a lookup label. `VariableRegistry` stores a borrowed pointer; it does
  not own, type-check or keep the pointed-to value alive.
- **Persistence:** bytes survive process exit; a numeric address does not become
  meaningful in the next process.
- **Durability:** acknowledged changes survive the specified crash model; a
  successful write or mapping alone does not establish it.
- **Bytes versus interpretation:** byte storage avoids mandatory text conversion,
  but integers still have byte order and records can contain padding and pointers.
  A durable format must define what is stored and how references are rebuilt.

## 11. Pressure points

1. **Indices vs handles.** Keeping both raw indices and handles is pragmatic but
   leaves a footgun. If identity should be everywhere, demote the raw index API to
   `pub(crate)`.
2. **C row identity is explicit.** `RowPool` exposes bytes and owner-tagged
   handles; it does not monomorphize arbitrary Rust types for C or attach
   ecosystem type ids.
3. **Mapping is a choice, not a readiness badge.** A fixed view cannot grow; a
   growable view reserves address space. Neither is a transaction, and page faults
   can block, so mapped storage is not a hot-path latency guarantee.
4. **Two toolchains.** Cargo and b both build; the seam is hand-written. The rule
   that keeps it honest is: C owns the stable ABI, Rust owns new typed storage
   behind it.

## 12. Scope and Limitations

**Scope:** the Rust vocabulary and storage surface above the C floor, plus the C
ABI that exposes selected storage to C consumers. Storage is aligned flat bytes,
not `Box`-per-row or arbitrary Rust object construction.

**Deliberately not covered:**

- No native allocator replacement: `re` owns `Memory_*`, arenas and transient
  scratch.
- No consumer rewrite: C keeps its language and logic; Rust owns the selected
  storage.
- No GPU or UI work: Graphvex and the R3+ tiers own those.
- No database policy: Darkbase owns transactions, recovery and durability.

**Known limits and gaps:**

- C calls the canonical engine C ABI and never casts Rust atomics to C `_Atomic`;
  no C/Rust atomic-layout compatibility is assumed.
- All non-trivial calls require external serialization; arbitrary or stale
  pointers are outside the ABI contract.
- Cold construction may abort on OOM; there is no general panic catcher.
- The current toolchain sanitizer and macOS floor gaps are stated in Current
  State.

## 13. Glossary

- **C floor** — `lib`: `vex.h` and the scalar vocabulary.
- **owner** — an opaque Rust value C holds by pointer and releases through its
  destructor.
- **stable row** — a stored element whose address does not change while it lives.
- **generation** — a counter bumped when a slot is freed, so old handles go stale.
- **reservation** — virtual address space mapped once; the committed length grows
  inside it.
- **seam** — the C/Rust boundary and its rules.

---

*Review boundary: this document was written after the Rust move. It was checked
against `lib.rs`, `primitive.rs`, `Cargo.toml`, `build.rs`,
`include/relational_engine/` and the crate README. The C side was checked against
`vex/re/nio/mem.h` and `vex/re/re.h`. This is not a complete fault, concurrency,
cross-platform or oldest-supported-macOS audit.*
