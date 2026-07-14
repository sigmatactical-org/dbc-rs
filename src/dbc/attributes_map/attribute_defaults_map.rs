//! [`AttributeDefaultsMap`].

#[allow(unused_imports)]
use super::*;
use crate::attribute::AttributeValue;

/// Storage for attribute default values (BA_DEF_DEF_).
///
/// Default values are used when an attribute is not explicitly assigned
/// to an object.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AttributeDefaultsMap {
    defaults: DefaultsMap,
}
impl AttributeDefaultsMap {
    /// Create a new empty attribute defaults map.
    #[allow(dead_code)]
    pub(crate) fn new() -> Self {
        Self {
            defaults: DefaultsMap::new(),
        }
    }

    /// Create from a BTreeMap of defaults.
    pub(crate) fn from_map(defaults: DefaultsMap) -> Self {
        Self { defaults }
    }

    /// Get an iterator over all attribute defaults.
    #[inline]
    #[must_use = "iterator is lazy and does nothing unless consumed"]
    pub fn iter(&self) -> impl Iterator<Item = (&str, &AttributeValue)> {
        self.defaults.iter().map(|(name, value)| (name.as_str(), value))
    }

    /// Get the number of attribute defaults.
    #[inline]
    #[must_use = "return value should be used"]
    pub fn len(&self) -> usize {
        self.defaults.len()
    }

    /// Returns `true` if there are no attribute defaults.
    #[inline]
    #[must_use = "return value should be used"]
    pub fn is_empty(&self) -> bool {
        self.defaults.is_empty()
    }

    /// Get the default value for an attribute by name.
    #[inline]
    #[must_use = "return value should be used"]
    pub fn get(&self, name: &str) -> Option<&AttributeValue> {
        self.defaults.iter().find(|(n, _)| n.as_str() == name).map(|(_, v)| v)
    }
}
