//! [`AttributeDefinition`].

#[allow(unused_imports)]
use super::*;
use crate::compat::Name;

/// Attribute definition from BA_DEF_.
///
/// Defines an attribute's name, target object type, and value type.
///
/// # Example
///
/// ```text
/// BA_DEF_ BO_ "GenMsgCycleTime" INT 0 10000;
/// ```
///
/// This defines a message attribute named "GenMsgCycleTime" with
/// integer values in range 0-10000.
#[derive(Debug, Clone, PartialEq)]
pub struct AttributeDefinition {
    /// Attribute name (quoted identifier in DBC)
    pub(crate) name: Name,
    /// Target object type (Network, Node, Message, Signal)
    pub(crate) object_type: AttributeObjectType,
    /// Value type specification
    pub(crate) value_type: AttributeValueType,
}
impl AttributeDefinition {
    /// Creates a new attribute definition.
    #[inline]
    pub(crate) fn new(
        name: Name,
        object_type: AttributeObjectType,
        value_type: AttributeValueType,
    ) -> Self {
        Self {
            name,
            object_type,
            value_type,
        }
    }
}
