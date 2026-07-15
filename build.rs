use std::env;

/// Check if a number is a power of 2
fn is_power_of_2(n: usize) -> bool {
    n > 0 && (n & (n - 1)) == 0
}

/// Calculate the next power of 2 greater than or equal to n
fn next_power_of_2(n: usize) -> usize {
    if n == 0 {
        return 1;
    }
    if is_power_of_2(n) {
        return n;
    }
    let mut v = n;
    v -= 1;
    v |= v >> 1;
    v |= v >> 2;
    v |= v >> 4;
    v |= v >> 8;
    v |= v >> 16;
    #[cfg(target_pointer_width = "64")]
    {
        v |= v >> 32;
    }
    v + 1
}

fn main() {
    // Enforce that either alloc or heapless feature must be enabled
    // Check Cargo feature flags via CARGO_FEATURE_* environment variables
    // (Note: hyphens become underscores, all uppercase)
    let has_alloc = env::var("CARGO_FEATURE_ALLOC").is_ok();
    let has_heapless = env::var("CARGO_FEATURE_HEAPLESS").is_ok();
    let has_std = env::var("CARGO_FEATURE_STD").is_ok();
    let has_attributes = env::var("CARGO_FEATURE_ATTRIBUTES").is_ok();

    // std includes alloc, so we only need to check if neither alloc nor heapless is enabled
    // Note: This check provides a better error message, but mayheap will also enforce this
    if !has_alloc && !has_heapless && !has_std {
        eprintln!("error: Either the `alloc` or `heapless` feature must be enabled");
        eprintln!("\ndbc-rs requires one of the following features:");
        eprintln!("  - `alloc`: Heap-allocated collections via alloc crate");
        eprintln!("  - `heapless`: Stack-allocated, bounded collections");
        eprintln!("  - `std`: Includes alloc + standard library features (default)");
        eprintln!("\nAdd to Cargo.toml:");
        eprintln!("  [dependencies]");
        eprintln!(
            "  dbc-rs = {{ version = \"...\", default-features = false, features = [\"alloc\"] }}"
        );
        eprintln!("  # OR");
        eprintln!(
            "  dbc-rs = {{ version = \"...\", default-features = false, features = [\"heapless\"] }}"
        );
        std::process::exit(1);
    }

    // Allow override of MAX_SIGNALS_PER_MESSAGE via environment variable
    let max_signals = env::var("DBC_MAX_SIGNALS_PER_MESSAGE")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(8); // Default to 8 signals/message — embedded-sane; override for big DBCs

    // Allow override of MAX_MESSAGES via environment variable
    let max_messages = env::var("DBC_MAX_MESSAGES")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(16); // Default to 16 messages — the 8192 default made Dbc gigabytes by value (issue #2)

    // Allow override of MAX_NODES via environment variable
    let max_nodes = env::var("DBC_MAX_NODES")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(8); // Default to 8 nodes

    // Allow override of MAX_VALUE_DESCRIPTIONS via environment variable
    let max_value_descriptions = env::var("DBC_MAX_VALUE_DESCRIPTIONS")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(16); // Default to 16 value descriptions per signal

    // Allow override of MAX_NAME_SIZE via environment variable
    let max_name_size = env::var("DBC_MAX_NAME_SIZE")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(32); // Default to 32 (per DBC specification)

    // Allow override of MAX_EXTENDED_MULTIPLEXING via environment variable
    let max_extended_multiplexing = env::var("DBC_MAX_EXTENDED_MULTIPLEXING")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(16); // Default to 16 extended-multiplexing entries

    // Attribute limits (only when attributes feature is enabled)
    let (max_attribute_definitions, max_attribute_values, max_attribute_enum_values) =
        if has_attributes {
            let max_attribute_definitions = env::var("DBC_MAX_ATTRIBUTE_DEFINITIONS")
                .ok()
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(32);

            let max_attribute_values = env::var("DBC_MAX_ATTRIBUTE_VALUES")
                .ok()
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(16);

            let max_attribute_enum_values = env::var("DBC_MAX_ATTRIBUTE_ENUM_VALUES")
                .ok()
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(64);

            (
                Some(max_attribute_definitions),
                Some(max_attribute_values),
                Some(max_attribute_enum_values),
            )
        } else {
            (None, None, None)
        };

    // Validate that all values are powers of 2 when heapless feature is enabled
    // heapless::Vec, heapless::String, and heapless::FnvIndexMap require power-of-2 capacities
    if has_heapless {
        let mut heapless_constants: Vec<(&str, usize, &str)> = vec![
            ("DBC_MAX_MESSAGES", max_messages, "MAX_MESSAGES"),
            (
                "DBC_MAX_SIGNALS_PER_MESSAGE",
                max_signals,
                "MAX_SIGNALS_PER_MESSAGE",
            ),
            ("DBC_MAX_NODES", max_nodes, "MAX_NODES"),
            ("DBC_MAX_NAME_SIZE", max_name_size, "MAX_NAME_SIZE"),
            (
                "DBC_MAX_EXTENDED_MULTIPLEXING",
                max_extended_multiplexing,
                "MAX_EXTENDED_MULTIPLEXING",
            ),
        ];

        // Add attribute constants only when feature is enabled
        if let Some(val) = max_attribute_definitions {
            heapless_constants.push((
                "DBC_MAX_ATTRIBUTE_DEFINITIONS",
                val,
                "MAX_ATTRIBUTE_DEFINITIONS",
            ));
        }
        if let Some(val) = max_attribute_values {
            heapless_constants.push(("DBC_MAX_ATTRIBUTE_VALUES", val, "MAX_ATTRIBUTE_VALUES"));
        }
        if let Some(val) = max_attribute_enum_values {
            heapless_constants.push((
                "DBC_MAX_ATTRIBUTE_ENUM_VALUES",
                val,
                "MAX_ATTRIBUTE_ENUM_VALUES",
            ));
        }

        for (env_var, value, const_name) in heapless_constants.iter() {
            if !is_power_of_2(*value) {
                eprintln!("error: {const_name} must be a power of 2 when using `heapless` feature",);
                eprintln!("  Current value: {value} (set via {env_var}={value})");
                eprintln!(
                    "  {const_name} is used with heapless collections which require power-of-2 capacities.",
                );
                eprintln!(
                    "\nValid power-of-2 values: 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, ..."
                );
                eprintln!("\nExample: Set {env_var} to a power of 2:");
                let np = next_power_of_2(*value);
                eprintln!("  {env_var}={np} cargo build ...");
                std::process::exit(1);
            }
        }
    }

    // Write the constants to a file in OUT_DIR
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("limits.rs");

    let mut limits_content = format!(
        r#"#[allow(dead_code)]
pub const MAX_SIGNALS_PER_MESSAGE: usize = {max_signals};
#[allow(dead_code)]
pub const MAX_MESSAGES: usize = {max_messages};
#[allow(dead_code)]
pub const MAX_NODES: usize = {max_nodes};
#[allow(dead_code)]
pub const MAX_VALUE_DESCRIPTIONS: usize = {max_value_descriptions};
#[allow(dead_code)]
pub const MAX_NAME_SIZE: usize = {max_name_size};
#[allow(dead_code)]
pub const MAX_EXTENDED_MULTIPLEXING: usize = {max_extended_multiplexing};
"#,
    );

    // Add attribute constants only when feature is enabled
    if let (Some(attr_defs), Some(attr_vals), Some(attr_enums)) = (
        max_attribute_definitions,
        max_attribute_values,
        max_attribute_enum_values,
    ) {
        limits_content.push_str(&format!(
            r#"#[allow(dead_code)]
pub const MAX_ATTRIBUTE_DEFINITIONS: usize = {attr_defs};
#[allow(dead_code)]
pub const MAX_ATTRIBUTE_VALUES: usize = {attr_vals};
#[allow(dead_code)]
pub const MAX_ATTRIBUTE_ENUM_VALUES: usize = {attr_enums};
"#,
        ));
    }

    // Limit-dependent error messages, so errors report the configured limit
    // instead of a stale hardcoded number. Always emitted (the attribute
    // messages fall back to the defaults when the feature is off — they are
    // unreachable then, but the lang module still references them).
    let attr_defs = max_attribute_definitions.unwrap_or(32);
    let attr_vals = max_attribute_values.unwrap_or(16);
    let attr_enums = max_attribute_enum_values.unwrap_or(64);
    let max_receivers = max_nodes - 1;
    limits_content.push_str(&format!(
        r#"#[allow(dead_code)]
pub(crate) const LIMIT_MSG_NODES_TOO_MANY: &str = "Too many nodes: maximum allowed is {max_nodes} (build-time DBC_MAX_NODES)";
#[allow(dead_code)]
pub(crate) const LIMIT_MSG_MESSAGES_TOO_MANY: &str = "Too many messages: maximum allowed is {max_messages} (build-time DBC_MAX_MESSAGES)";
#[allow(dead_code)]
pub(crate) const LIMIT_MSG_MESSAGE_TOO_MANY_SIGNALS: &str = "Too many signals: maximum allowed is {max_signals} per message (build-time DBC_MAX_SIGNALS_PER_MESSAGE)";
#[allow(dead_code)]
pub(crate) const LIMIT_MSG_SIGNAL_RECEIVERS_TOO_MANY: &str = "Too many receiver nodes: maximum allowed is {max_receivers} per signal (build-time DBC_MAX_NODES - 1)";
#[allow(dead_code)]
pub(crate) const LIMIT_MSG_EXTENDED_MULTIPLEXING_TOO_MANY: &str = "Too many extended multiplexing entries: maximum allowed is {max_extended_multiplexing} per DBC file (build-time DBC_MAX_EXTENDED_MULTIPLEXING)";
#[allow(dead_code)]
pub(crate) const LIMIT_MSG_VALUE_DESCRIPTIONS_TOO_MANY: &str = "Too many value descriptions: maximum allowed is {max_value_descriptions} (build-time DBC_MAX_VALUE_DESCRIPTIONS)";
#[allow(dead_code)]
pub(crate) const LIMIT_MSG_ATTRIBUTE_DEFINITIONS_TOO_MANY: &str = "Too many attribute definitions: maximum allowed is {attr_defs} per DBC file (build-time DBC_MAX_ATTRIBUTE_DEFINITIONS)";
#[allow(dead_code)]
pub(crate) const LIMIT_MSG_ATTRIBUTE_VALUES_TOO_MANY: &str = "Too many attribute values: maximum allowed is {attr_vals} per DBC file (build-time DBC_MAX_ATTRIBUTE_VALUES)";
#[allow(dead_code)]
pub(crate) const LIMIT_MSG_ATTRIBUTE_ENUM_VALUES_TOO_MANY: &str = "Too many enum values in attribute definition: maximum allowed is {attr_enums} (build-time DBC_MAX_ATTRIBUTE_ENUM_VALUES)";
"#,
    ));

    std::fs::write(&dest_path, limits_content).unwrap();

    // Rebuild if the environment variables change
    println!("cargo:rerun-if-env-changed=DBC_MAX_SIGNALS_PER_MESSAGE");
    println!("cargo:rerun-if-env-changed=DBC_MAX_MESSAGES");
    println!("cargo:rerun-if-env-changed=DBC_MAX_NODES");
    println!("cargo:rerun-if-env-changed=DBC_MAX_VALUE_DESCRIPTIONS");
    println!("cargo:rerun-if-env-changed=DBC_MAX_NAME_SIZE");
    println!("cargo:rerun-if-env-changed=DBC_MAX_EXTENDED_MULTIPLEXING");
    println!("cargo:rerun-if-env-changed=DBC_MAX_ATTRIBUTE_DEFINITIONS");
    println!("cargo:rerun-if-env-changed=DBC_MAX_ATTRIBUTE_VALUES");
    println!("cargo:rerun-if-env-changed=DBC_MAX_ATTRIBUTE_ENUM_VALUES");
}
