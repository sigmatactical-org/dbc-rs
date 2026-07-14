//! [`AttributeValueType`].

#[allow(unused_imports)]
use super::*;

/// Attribute value type specification from BA_DEF_.
///
/// Defines the type and constraints for an attribute's values.
#[derive(Debug, Clone, PartialEq)]
pub enum AttributeValueType {
    /// Integer type with min/max range: `INT min max`
    Int {
        /// Minimum allowed value
        min: i64,
        /// Maximum allowed value
        max: i64,
    },
    /// Hexadecimal integer type with min/max range: `HEX min max`
    Hex {
        /// Minimum allowed value
        min: i64,
        /// Maximum allowed value
        max: i64,
    },
    /// Float type with min/max range: `FLOAT min max`
    Float {
        /// Minimum allowed value
        min: f64,
        /// Maximum allowed value
        max: f64,
    },
    /// String type: `STRING`
    String,
    /// Enumeration type: `ENUM "val1","val2",...`
    Enum {
        /// List of valid enum values
        values: EnumValues,
    },
}
