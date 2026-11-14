# rangoon — Repo-Local Living Preferences

## Constitution Link

gist.md (`../../metadata/gists/preferences/gist.md`) is the compressed law list; all universal laws bind this repository. Read one on demand: `./tools/prefs law "<title>"`. Universal laws are inherited, never restated.

## Tier and Ownership

**Tier:** `lifetime` — the Rust peer of the C floor, not a second floor.
**Owns:** the Rust projection of the scalar vocabulary (`d64`, `bytes`, `any`, `const_any`, `fnptr`); the Rust storage surface (`nio`, `io`, `primitives`, `variable`, `struct`, `compress`, `virtual`, `ffi`); the C name-search primitive `search/primitives/name_search.c`; and the C ABI headers under `include/` that declare that surface.
**Does not own:** the vocabulary itself — the C floor (`lib/lang/primitive.h`) owns the names; the C allocator and its ABI — `vex/re` owns them; or any behavior a lower tier already owns. It references the C contract; it never re-implements it. It calls the C name-search primitive it ships rather than importing comparison code.

The C ABI keeps its `relational_engine/` namespace and `re_*` symbols through the
move; renaming that public contract is a separate change, not part of the
migration.

## Directory Headers

Rust has no directory header. Modules mirror the C layout: `primitive.rs` is the
Rust `lang/primitive.h`, one module per contract, re-exported from `lib.rs`.
Each owned type gets its own file (the One Rust Class Per File Law).

## Exclusive Laws

| Law Title | Scope | Enforcement |
| **Vocabulary Projection Law** | `primitive.rs` | Mandatory for `rangoon` |
| **Cross-Language Atomic Access Law** | Shared primitives and strings | Mandatory for `rangoon` |
| **One Rust Class Per File Law** | Rust declarations | Mandatory for `rangoon` |
| **CamelCase Rust Constructor Macro Law** | Construction spelling | Mandatory for `rangoon` |
| **Rust Storage Boundary Law** | Rust storage; C allocator seam | Mandatory for `rangoon` |
| **Stable Row and Variable Binding Law** | Chunks, bindings, 32-byte ABI | Mandatory for `rangoon` |
| **Fixed File Mapping Law** | Fixed-extent file-backed byte views | Mandatory for `rangoon` |
| **Growable File Mapping Law** | Reservation-backed growing byte views | Mandatory for `rangoon` |
| **Physical File Reservation Law** | Create-new byte files | Mandatory for `rangoon` |

### Vocabulary Projection Law

* A name the C floor already owns is projected, never redefined: `d64`, `bytes`, `any`, `const_any`, `fnptr` are aliases over Rust's own types.
* A token that is already the name (`u8`..`u64`, `i8`..`i64`, `f32`, `bool`) gets no alias; two spellings of one type is the defect the projection exists to prevent.
* `b16` and the float-atomic spellings (`af32`, `ad64`, `ald128`) are owned types, not aliases: Rust core has no bfloat16 and no float atomics.
* `pointer` is not aliased: `*mut T`/`*const T` is a pointer and `usize` is an address; a third name hides which one a call site means.
* Every projection carries a compile-time width assertion mirroring the C floor's `_Static_assert`; the two halves prove the same arithmetic independently.

### Cross-Language Atomic Access Law

* Rust owns atomic storage; C calls the canonical engine C ABI and never casts Rust atomics to C `_Atomic` representations.
* Registration/release/destruction require exclusive access; existing atomic values permit concurrent atomic get/set while the registry stays frozen.
* Non-atomic getters cannot read atomic cells; wrong storage kinds reject with a defined error.
* Atomic Bytes use release stores/acquire loads; atomic strings publish immutable byte snapshots with release/acquire; readers never lock; writers try-lock and return busy.

### One Rust Class Per File Law

* One file per class: a class is its Rust struct plus its owning `impl` methods; the lowercase snake-case file name matches the class.
* Each implementation file defines at most one named struct or enum; public methods `pub fn`.
* Each enum has its own file; private helper records too.
* `mod.rs`/`lib.rs` declare/re-export modules and macros, not mixed behavioral types.

### CamelCase Rust Constructor Macro Law

* Constructor macros use CamelCase matching the object concept: `Memory!()` and `Bytes!()`.
* Ordinary Rust methods and C-facing primitive operations use snake_case (e.g. `get_atomic_byte`).
* Compatibility lowercase macros may remain for existing clients but never replace the canonical spelling in new examples.

### Rust Storage Boundary Law

* Rust storage is a distinct API above the C floor; it is not a silent allocator substitution for `vex/re`.
* `vex/re` owns the C allocator and its ABI; rangoon never re-implements `Memory_alloc`, `MemoryArena_alloc` or `Transient_alloc`.
* `search/primitives/name_search.c` is compiled in-repo by `build.rs`; it includes the C floor (`vex.h`, `debug/throw.h`) downward and owns no C allocator.
* The C ABI headers under `include/` are rangoon's contract; a C caller includes them and links the Cargo static library.

### Stable Row and Variable Binding Law

* `nio/chunk.rs` owns a fixed-capacity typed row allocation; `struct/chunked_list.rs` grows its directory without moving initialized rows.
* Geometry is chosen at construction (named default, caller override), validated for size/alignment, immutable thereafter.
* Rows are append-only, never deleted/reused in this version.
* `variable/variable_slot.rs` is a C-layout 32-byte binding: `[name: u8[24]][value pointer: 8 bytes]`, offsets 0/24 checked.
* Names are 1..23 ASCII bytes plus NUL and zero padding, folded lowercase; invalid input rejects without silent truncation or mutating prior state.
* Zero slots are unnamed/unbound; registry admission requires a valid name.
* `VariableRegistry` owns stable slot rows and rejects folded duplicate names.
* C `search/primitives/name_search` scans fixed 24-byte keys in borrowed leaves: no Vexspoke/host headers, no allocation; invalid spans admitted through `debug/throw.h`.

### Fixed File Mapping Law

* `FixedMappedFile` owns a fixed-extent existing regular-file mapping through memmap2; the owner retains the file until close/drop and unmaps before release.
* Read-only and shared read/write modes are explicit; empty files are valid open owners without an OS mapping.
* No implicit create/truncate/resize, C mapping ABI, internal dirty tracker, database format or typed reinterpretation.
* Opening is unsafe admission: the caller excludes external mutation/truncation and alias violations of writable exclusivity for the full mapping lifetime.
* Reads/writes validate bounds and arithmetic before access; rejection preserves bytes.
* Flush requests mapped-page writeback; sync requests file synchronization; both expose OS errors; neither claims atomic publication or power-loss transactions.

### Growable File Mapping Law

* `GrowableMappedFile` owns a fixed virtual RESERVATION over a file it creates or reopens, plus a committed length growing inside that reservation.
* Reservation is fixed at create, caller-chosen; mapping made once; growing the reservation is not offered.
* Committed length grows in caller-sized steps; only the step's blocks are allocated, never the whole reservation up front.
* Reserve step blocks, then raise the committed length; a failed growth leaves length and every byte unchanged.
* Byte access is bounded by committed length, never the reservation.
* Released ranges are reused via a free list of emptied page ranges.
* Always read/write; no read-only form: growth requires write access.

### Physical File Reservation Law

* `PreallocatedFile`: caller-sized nonzero, create-new regular byte storage.
* macOS F_PREALLOCATE/F_ALLOCATEALL and Linux fallocate reserve disk before success; EOF is never reservation and allocated OS blocks must cover the extent.
* Unsupported hosts/filesystems and allocation failures reject without sparse fallback.
* Unix mode starts at 0600; existing destinations/final symlinks reject unchanged.
* Creation-stage failure closes/removes the artifact; cleanup failure reports both errors; drop/close retains successful files.

## Repo-Local Extensions

* None — no managed exceptions to universal laws.

## Readiness Cross-Reference

* Feature readiness: rangoon readiness gist.
* Open blockers: ecosystem blockers gist.
* Proof lawbook: `tests/test-preferences.md` — `./tools/prefs test "<title>"`.
* Evidence ledger: `metadata/gists/test-checklist.md` — `python3 tools/test_checklist.py check`.
