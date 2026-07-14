//! [`SwitchValues`].

#[allow(unused_imports)]
use super::*;
use crate::{Error, Result};

/// Pre-allocated buffer for switch values during decode.
/// Uses fixed-size arrays to avoid heap allocation.
pub(crate) struct SwitchValues<'a> {
    /// Switch names (references to signal names, valid for decode lifetime)
    pub(crate) names: [Option<&'a str>; MAX_SWITCHES],
    /// Switch values corresponding to each name
    pub(crate) values: [u64; MAX_SWITCHES],
    /// Number of switches stored.
    pub(crate) count: usize,
}
impl<'a> SwitchValues<'a> {
    #[inline]
    pub(crate) const fn new() -> Self {
        Self {
            names: [None; MAX_SWITCHES],
            values: [0; MAX_SWITCHES],
            count: 0,
        }
    }

    /// Store a switch value. Returns Err if capacity exceeded.
    #[inline]
    pub(crate) fn push(&mut self, name: &'a str, value: u64) -> Result<()> {
        if self.count >= MAX_SWITCHES {
            return Err(Error::Decoding(Error::MESSAGE_TOO_MANY_SIGNALS));
        }
        let idx = self.count;
        self.names[idx] = Some(name);
        self.values[idx] = value;
        self.count += 1;
        Ok(())
    }

    /// Find switch value by name. O(n) where n is number of switches (typically 1-2).
    #[inline]
    pub(crate) fn get_by_name(&self, name: &str) -> Option<u64> {
        for i in 0..self.count {
            if self.names[i] == Some(name) {
                return Some(self.values[i]);
            }
        }
        None
    }

    /// Check if any switch has the given value.
    #[inline]
    pub(crate) fn any_has_value(&self, target: u64) -> bool {
        for i in 0..self.count {
            if self.values[i] == target && self.names[i].is_some() {
                return true;
            }
        }
        false
    }
}
