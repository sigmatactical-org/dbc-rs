//! [`AttributeValuesMap`].

#[allow(unused_imports)]
use super::*;
use crate::attribute::{AttributeTarget, AttributeValue};

/// Storage for attribute values (BA_).
///
/// Attribute values are the actual values assigned to specific DBC objects.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AttributeValuesMap {
    values: ValuesMap,
}
impl AttributeValuesMap {
    /// Create a new empty attribute values map.
    #[allow(dead_code)]
    pub(crate) fn new() -> Self {
        Self {
            values: ValuesMap::new(),
        }
    }

    /// Create from a BTreeMap of values.
    pub(crate) fn from_map(values: ValuesMap) -> Self {
        Self { values }
    }

    /// Get an iterator over all attribute values.
    #[inline]
    #[must_use = "iterator is lazy and does nothing unless consumed"]
    pub fn iter(&self) -> impl Iterator<Item = ((&str, &AttributeTarget), &AttributeValue)> {
        self.values
            .iter()
            .map(|((name, target), value)| ((name.as_str(), target), value))
    }

    /// Get the number of attribute values.
    #[inline]
    #[must_use = "return value should be used"]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns `true` if there are no attribute values.
    #[inline]
    #[must_use = "return value should be used"]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Get an attribute value by name and target.
    #[inline]
    #[must_use = "return value should be used"]
    pub fn get(&self, name: &str, target: &AttributeTarget) -> Option<&AttributeValue> {
        self.values
            .iter()
            .find(|((n, t), _)| n.as_str() == name && t == target)
            .map(|(_, v)| v)
    }

    /// Get a network-level attribute value by name.
    #[inline]
    #[must_use = "return value should be used"]
    pub fn get_network(&self, name: &str) -> Option<&AttributeValue> {
        self.get(name, &AttributeTarget::Network)
    }

    /// Get a node attribute value by node name and attribute name.
    #[inline]
    #[must_use = "return value should be used"]
    pub fn get_node(&self, node_name: &str, attr_name: &str) -> Option<&AttributeValue> {
        self.values
            .iter()
            .find(|((n, t), _)| {
                n.as_str() == attr_name
                    && matches!(t, AttributeTarget::Node(name) if name.as_str() == node_name)
            })
            .map(|(_, v)| v)
    }

    /// Get a message attribute value by message ID and attribute name.
    #[inline]
    #[must_use = "return value should be used"]
    pub fn get_message(&self, message_id: u32, attr_name: &str) -> Option<&AttributeValue> {
        self.values
            .iter()
            .find(|((n, t), _)| {
                n.as_str() == attr_name
                    && matches!(t, AttributeTarget::Message(id) if *id == message_id)
            })
            .map(|(_, v)| v)
    }

    /// Get a signal attribute value by message ID, signal name, and attribute name.
    #[inline]
    #[must_use = "return value should be used"]
    pub fn get_signal(
        &self,
        message_id: u32,
        signal_name: &str,
        attr_name: &str,
    ) -> Option<&AttributeValue> {
        self.values
            .iter()
            .find(|((n, t), _)| {
                n.as_str() == attr_name
                    && matches!(t, AttributeTarget::Signal(msg_id, sig_name)
                        if *msg_id == message_id && sig_name.as_str() == signal_name)
            })
            .map(|(_, v)| v)
    }
}
