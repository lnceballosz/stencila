// Generated file; do not edit. See `schema-gen` crate.

use crate::prelude::*;

use super::inline::Inline;
use super::string::String;

/// Superscripted content.
#[skip_serializing_none]
#[derive(Debug, SmartDefault, Clone, PartialEq, Serialize, Deserialize, StripNode, HtmlCodec, TextCodec, Read, Write)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
#[html(elem = "sup")]
pub struct Superscript {
    /// The type of this item
    pub r#type: MustBe!("Superscript"),

    /// The identifier for this item
    #[strip(id)]
    #[html(attr = "id")]
    pub id: Option<String>,

    /// The content that is marked.
    pub content: Vec<Inline>,
}
impl Superscript {
    pub fn new(content: Vec<Inline>) -> Self {
        Self {
            content,
            ..Default::default()
        }
    }
}
