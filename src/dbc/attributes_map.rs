//! Storage wrappers for DBC attributes.
//!
//! Provides storage for:
//! - Attribute definitions (BA_DEF_)
//! - Attribute defaults (BA_DEF_DEF_)
//! - Attribute values (BA_)

mod attribute_defaults_map;
mod attribute_definitions_map;
mod attribute_values_map;
pub use attribute_defaults_map::AttributeDefaultsMap;
pub use attribute_definitions_map::AttributeDefinitionsMap;
pub use attribute_values_map::AttributeValuesMap;

use crate::{
    MAX_ATTRIBUTE_DEFINITIONS, MAX_ATTRIBUTE_VALUES,
    attribute::{AttributeTarget, AttributeValue},
    compat::{BTreeMap, Name},
};

/// Type alias for attribute defaults map (attribute_name -> default_value)
type DefaultsMap = BTreeMap<Name, AttributeValue, MAX_ATTRIBUTE_DEFINITIONS>;

/// Type alias for attribute values map ((attribute_name, target) -> value)
type ValuesMap = BTreeMap<(Name, AttributeTarget), AttributeValue, MAX_ATTRIBUTE_VALUES>;
