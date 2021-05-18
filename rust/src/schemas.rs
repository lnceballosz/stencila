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
use serde_json::Value as JsonValue;

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
pub fn generate<Type>() -> Result<JsonValue>
where
    Type: JsonSchema,
{
    let schema = generator().into_root_schema_for::<Type>();
    let schema = serde_json::to_value(schema)?;

    // Modify `$id`, `title` and `description` for compatibility with TypeScript
    // type generation.
    // See https://github.com/stencila/stencila/pull/929#issuecomment-842623228
    fn modify(value: JsonValue) -> JsonValue {
        if let JsonValue::Object(object) = value {
            let mut modified = serde_json::Map::<String, JsonValue>::new();

            // Copy over modified child properties
            for (key, child) in &object {
                modified.insert(key.clone(), modify(child.clone()));
            }

            // For `type:object` schemas, including sub-schemas..
            if let Some(value) = object.get("type") {
                if value == &serde_json::to_value("object").unwrap() {
                    // Put any `title` into `$id`
                    if let Some(title) = object.get("title") {
                        modified.insert("$id".into(), title.clone());
                    }
                    // Parse any `description` and if multi-line, put
                    // the first "paragraph" into the `title`
                    if let Some(description) = object.get("description") {
                        if let JsonValue::String(description) = description {
                            let paras = description.split("\n\n").collect::<Vec<&str>>();
                            if paras.len() > 1 {
                                modified.insert("title".into(), JsonValue::String(paras[0].into()));
                                modified.insert(
                                    "description".into(),
                                    JsonValue::String(paras[1..].join("\n\n")),
                                );
                            }
                        }
                    }
                }
            }
            JsonValue::Object(modified)
        } else {
            value
        }
    }
    let schema = modify(schema);
    Ok(schema)
}
