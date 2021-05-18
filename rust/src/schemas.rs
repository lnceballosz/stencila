//! Functions for consistently generating JSON Schemata from
//! internal Rust `struct`s.
//!
//! Not to be confused with the `stencila-schema` crate which
//! provides Rust `struct`s generated from Stencila's JSON Schema ;)

use eyre::Result;
use schemars::{
    gen::{SchemaGenerator, SchemaSettings},
    JsonSchema,
};

/// Create a `schemars` JSON Schema generator
///
/// Having a shared generator allow for consistent settings
/// for how JSON Schemas are produced across modules.
pub fn generator() -> SchemaGenerator {
    let settings = SchemaSettings::draft2019_09().with(|settings| {
        settings.option_add_null_type = false;
        settings.inline_subschemas = true;
    });
    settings.into_generator()
}

/// Generate a JSON Schema for a type using the generator
pub fn generate<Type>() -> Result<serde_json::Value>
where
    Type: JsonSchema,
{
    let schema = generator().into_root_schema_for::<Type>();
    let schema = serde_json::to_value(schema)?;
    Ok(schema)
}
