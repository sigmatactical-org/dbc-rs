//! [`AttributeValue`].

#[allow(unused_imports)]
use super::*;

/// Concrete attribute value from BA_ or BA_DEF_DEF_.
///
/// Represents the actual value assigned to an attribute.
#[derive(Debug, Clone, PartialEq)]
pub enum AttributeValue {
    /// Integer value (for INT and HEX types)
    Int(i64),
    /// Floating-point value (for FLOAT type)
    Float(f64),
    /// String value (for STRING and ENUM types)
    String(AttributeString),
}
