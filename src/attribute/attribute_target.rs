//! [`AttributeTarget`].

#[allow(unused_imports)]
use super::*;
use crate::compat::Name;

/// Target object reference for attribute value assignment.
///
/// Identifies which specific object an attribute value applies to.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AttributeTarget {
    /// Network/database level (global)
    Network,
    /// Node by name
    Node(Name),
    /// Message by ID
    Message(u32),
    /// Signal by (message_id, signal_name)
    Signal(u32, Name),
}
