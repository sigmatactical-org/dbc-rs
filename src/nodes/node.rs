//! [`Node`].

#[allow(unused_imports)]
use super::*;
use crate::compat::{Comment, Name};

/// Represents a single node (ECU) with its name and optional comment.
#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct Node {
    name: Name,
    comment: Option<Comment>,
}
impl Node {
    /// Creates a new node with the given name and no comment.
    #[inline]
    pub(crate) fn new(name: Name) -> Self {
        Self {
            name,
            comment: None,
        }
    }

    /// Creates a new node with the given name and comment.
    #[inline]
    #[cfg(feature = "std")]
    pub(crate) fn with_comment(name: Name, comment: Option<Comment>) -> Self {
        Self { name, comment }
    }

    /// Returns the node name.
    #[inline]
    #[must_use = "return value should be used"]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the node comment, if any.
    #[inline]
    #[must_use = "return value should be used"]
    pub fn comment(&self) -> Option<&str> {
        self.comment.as_ref().map(|c| c.as_str())
    }

    /// Sets the comment for this node.
    #[inline]
    pub(crate) fn set_comment(&mut self, comment: Comment) {
        self.comment = Some(comment);
    }
}
