//! Attribute definitions and values for DBC files.
//!
//! This module provides types for parsing and representing DBC attributes:
//! - `BA_DEF_` - Attribute definitions
//! - `BA_DEF_DEF_` - Attribute default values
//! - `BA_` - Attribute value assignments
//!
//! # DBC Attribute System
//!
//! Attributes in DBC files provide metadata for network objects:
//! - Network/database level (global attributes)
//! - Node attributes (BU_)
//! - Message attributes (BO_)
//! - Signal attributes (SG_)
//!
//! # Example
//!
//! ```text
//! BA_DEF_ BO_ "GenMsgCycleTime" INT 0 10000;
//! BA_DEF_DEF_ "GenMsgCycleTime" 100;
//! BA_ "GenMsgCycleTime" BO_ 256 50;
//! ```

mod attribute_definition;
mod attribute_object_type;
mod attribute_target;
mod attribute_value;
mod attribute_value_type;
pub use attribute_definition::AttributeDefinition;
pub use attribute_object_type::AttributeObjectType;
pub use attribute_target::AttributeTarget;
pub use attribute_value::AttributeValue;
pub use attribute_value_type::AttributeValueType;

mod impls;
pub(crate) mod parse;
#[cfg(feature = "std")]
mod std;

#[cfg(feature = "std")]
mod builder;

use crate::compat::{Name, String, Vec};

#[cfg(feature = "std")]
pub use builder::AttributeDefinitionBuilder;

/// Maximum size for attribute string values.
pub const MAX_ATTRIBUTE_STRING_SIZE: usize = 256;

/// Type alias for attribute string values.
pub type AttributeString = String<MAX_ATTRIBUTE_STRING_SIZE>;

/// Type alias for enum value list in attribute definitions.
pub type EnumValues = Vec<Name, { crate::MAX_ATTRIBUTE_ENUM_VALUES }>;

/// Type alias for attribute definitions collection.
pub type AttributeDefinitions = Vec<AttributeDefinition, { crate::MAX_ATTRIBUTE_DEFINITIONS }>;
