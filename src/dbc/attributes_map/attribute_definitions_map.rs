//! [`AttributeDefinitionsMap`].

#[allow(unused_imports)]
use super::*;
use crate::attribute::{
    AttributeDefinition, AttributeDefinitions as AttrDefsVec, AttributeObjectType,
};

/// Storage for attribute definitions (BA_DEF_).
///
/// Attribute definitions specify the name, object type, and value constraints
/// for attributes that can be assigned to DBC objects.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AttributeDefinitionsMap {
    definitions: AttrDefsVec,
}
impl AttributeDefinitionsMap {
    /// Create a new empty attribute definitions map.
    #[allow(dead_code)]
    pub(crate) fn new() -> Self {
        Self {
            definitions: AttrDefsVec::new(),
        }
    }

    /// Create from a vector of definitions.
    pub(crate) fn from_vec(definitions: AttrDefsVec) -> Self {
        Self { definitions }
    }

    /// Get an iterator over all attribute definitions.
    #[inline]
    #[must_use = "iterator is lazy and does nothing unless consumed"]
    pub fn iter(&self) -> impl Iterator<Item = &AttributeDefinition> {
        self.definitions.as_slice().iter()
    }

    /// Get the number of attribute definitions.
    #[inline]
    #[must_use = "return value should be used"]
    pub fn len(&self) -> usize {
        self.definitions.len()
    }

    /// Returns `true` if there are no attribute definitions.
    #[inline]
    #[must_use = "return value should be used"]
    pub fn is_empty(&self) -> bool {
        self.definitions.is_empty()
    }

    /// Get an attribute definition by name.
    #[inline]
    #[must_use = "return value should be used"]
    pub fn get(&self, name: &str) -> Option<&AttributeDefinition> {
        self.definitions.as_slice().iter().find(|def| def.name() == name)
    }

    /// Get an attribute definition by name and object type.
    #[inline]
    #[must_use = "return value should be used"]
    pub fn get_for_type(
        &self,
        name: &str,
        object_type: AttributeObjectType,
    ) -> Option<&AttributeDefinition> {
        self.definitions
            .as_slice()
            .iter()
            .find(|def| def.name() == name && def.object_type() == object_type)
    }
}
