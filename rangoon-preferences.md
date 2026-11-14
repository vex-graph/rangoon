# rangoon — Repo-Local Living Preferences

## Constitution Link

gist.md (`../../metadata/gists/preferences/gist.md`) is the compressed law list; all universal laws bind this repository. Read one on demand: `./tools/prefs law "<title>"`. Universal laws are inherited, never restated.

## Tier and Ownership

**Tier:** `lifetime` — the Rust peer of the C floor, not a second floor.
**Owns:** the Rust projection of the scalar vocabulary (`d64`, `bytes`, `any`, `const_any`, `fnptr`) and, over time, the Rust library surface the ecosystem consumes.
**Does not own:** the vocabulary itself — the C floor (`lib/lang/primitive.h`) owns the names; memory allocation — `vex/re` owns every runtime byte; or any behavior a lower tier already owns. It references the C contract; it never re-implements it.

## Directory Headers

Rust has no directory header. Modules mirror the C layout: `primitive.rs` is the
Rust `lang/primitive.h`, one module per contract, re-exported from `lib.rs`.
Each owned type gets its own file (the One Rust Class Per File Law).

## Exclusive Laws

| Law Title | Scope | Enforcement |
| **Vocabulary Projection Law** | `primitive.rs` | Mandatory for `rangoon` |

### Vocabulary Projection Law

* A name the C floor already owns is projected, never redefined: `d64`, `bytes`, `any`, `const_any`, `fnptr` are aliases over Rust's own types.
* A token that is already the name (`u8`..`u64`, `i8`..`i64`, `f32`, `bool`) gets no alias; two spellings of one type is the defect the projection exists to prevent.
* `b16` and the float-atomic spellings (`af32`, `ad64`, `ald128`) are owned types, not aliases: Rust core has no bfloat16 and no float atomics.
* `pointer` is not aliased: `*mut T`/`*const T` is a pointer and `usize` is an address; a third name hides which one a call site means.
* Every projection carries a compile-time width assertion mirroring the C floor's `_Static_assert`; the two halves prove the same arithmetic independently.

## Repo-Local Extensions

* None — no managed exceptions to universal laws.

## Readiness Cross-Reference

* Feature readiness: rangoon readiness gist.
* Open blockers: ecosystem blockers gist.
* Proof lawbook: `tests/test-preferences.md` — `./tools/prefs test "<title>"`.
* Evidence ledger: `metadata/gists/test-checklist.md` — `python3 tools/test_checklist.py check`.
