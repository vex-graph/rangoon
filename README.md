# rangoon
The Rust vocabulary and library surface of the vexgraph ecosystem.

**Ecosystem repository:** https://github.com/vex-graph/rangoon
The local checkout is `rust/`; its physical location does not change its tier.
It is the Rust peer of the C floor (`lib`), not a second floor.

## Current State

**Role:** lifetime (Rust projection). The crate projects the vocabulary the C
floor already owns (`lib/lang/primitive.h`) into Rust tokens, and is the home
for the ecosystem's Rust library surface as it moves out of `relational-engine`.

**Implemented:**
- `primitive`: `d64`, `bytes`, `any`, `const_any`, `fnptr` aliases, with
  compile-time width assertions mirroring the C floor's `_Static_assert`s.
- `u8`..`u64`, `i8`..`i64`, `f32`, `bool` need no alias: the token is already
  the name.

**Not implemented:** `b16` (owned bfloat16 type), the float-atomic spellings
(`af32`, `ad64`, `ald128`), and any storage or IO surface. The relational-engine
Rust modules still live in `vex/re/rust`; their move here is a later cycle.

## Scope and Limitations

- The crate defines no behavior for the scalar vocabulary; it is a naming
  projection, not an allocator, a storage engine or a wrapper.
- `bytes` is `usize`: the same width on the six-target floor, and the type
  Rust's standard library returns from `len()`.
- `fnptr` is `Option<unsafe extern "C" fn(any) -> any>`: the `Option` carries
  the null, and a call is `unsafe`, which C's spelling does not encode.
- `b16` and the float atomics are owned types, not aliases: Rust core has no
  bfloat16 and no float atomics.
- `pointer` is not aliased: Rust spells a pointer `*mut T`/`*const T` and an
  address `usize`; a third name would hide which one a call site means.
- Proven on macOS arm64 only; Linux and Windows are unproven for this crate.
