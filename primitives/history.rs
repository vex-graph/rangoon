//! Private primitive string retention ledger. Snapshots live until destruction.
use crate::annotation::overview;
use super::snapshot::Snapshot;

#[overview]
// INTENTIONAL(vex): Box<Snapshot> keeps each Snapshot header at a stable heap
// address when snapshots reallocates, so AtomicPtr<Snapshot> readers in current()
// never dereference a moved header.
#[allow(clippy::vec_box)]
pub(super) struct History {
    pub(super) snapshots: Vec<Box<Snapshot>>,
    pub(super) retained: usize,
}
