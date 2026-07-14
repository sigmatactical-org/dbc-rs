//! [`AttributeObjectType`].

#[allow(unused_imports)]
use super::*;

/// Object type that an attribute applies to.
///
/// Corresponds to the object_type in `BA_DEF_`:
/// - Empty (no prefix) = Network/database level
/// - `BU_` = Node
/// - `BO_` = Message
/// - `SG_` = Signal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AttributeObjectType {
    /// Network/database level attribute (no object prefix in BA_DEF_)
    #[default]
    Network,
    /// Node attribute (BU_ prefix)
    Node,
    /// Message attribute (BO_ prefix)
    Message,
    /// Signal attribute (SG_ prefix)
    Signal,
}
