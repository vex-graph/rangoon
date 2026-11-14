//! Explicit foreign interfaces, separate from native storage behavior.
pub mod block_table;
pub mod memory;
pub mod variable_registry;
pub mod row_pool;
pub mod fixed_mapped_file;
pub mod growable_mapped_file;
pub use variable_registry::{re_variables_new, re_variables_drop, re_variables_add,
    re_variables_find, re_variables_slot, re_variables_set_pointer};
// Preserve the original ffi::re_memory_* client paths.
pub use memory::{re_memory_copy, re_memory_drop, re_memory_new, re_memory_read};
pub use block_table::{re_blocks_new, re_blocks_new_with_limit, re_blocks_drop,
    re_blocks_issue, re_blocks_validate, re_blocks_resolve, re_blocks_release,
    re_blocks_generation, re_blocks_live, re_blocks_to_string, re_blocks_to_string_struct};
pub use memory::{re_memory_get_byte, re_memory_new_atomic_byte, re_memory_get_atomic_byte,
    re_memory_set_atomic_byte, re_memory_new_atomic_string, re_memory_get_atomic_string,
    re_memory_set_atomic_string};
