//! Private generation state for one payload address. `generation == 0` is
//! retired; `live` says whether the current generation is issued.
use crate::annotation::overview;

#[overview]
#[derive(Clone, Copy, Debug)]
pub(super) struct BlockSlot {
    pub(super) generation: u32,  // current generation; zero after the limit is exhausted
    pub(super) live: bool,       // issued and not yet released
}
