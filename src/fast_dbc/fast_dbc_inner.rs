//! [`FastDbcInner`].

#[allow(unused_imports)]
use super::*;
use crate::Dbc;
use decode::DecodePlan;
use hasher::FxHashMap;

/// Owned DBC plus the precomputed id maps and per-message decode plans.
pub(crate) struct FastDbcInner {
    /// The underlying DBC
    pub(crate) dbc: Dbc,
    /// Direct lookup table for standard CAN IDs (0-2047)
    /// Value is index into decode_plans, or usize::MAX if not present
    pub(crate) standard_ids: Box<[usize; MAX_STANDARD_ID]>,
    /// Hash map for extended CAN IDs and IDs >= 2048
    pub(crate) extended_ids: FxHashMap<u32, usize>,
    /// Pre-computed decode plans for each message
    pub(crate) decode_plans: Vec<DecodePlan>,
    /// Maximum signals in any single message
    pub(crate) max_signals: usize,
    /// Total signal count across all messages
    pub(crate) total_signals: usize,
}
