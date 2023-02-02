//! Generated file, do not edit

use crate::prelude::*;

use super::inline::Inline;
use super::string::String;

/// Subscripted content.
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "common::serde")]
pub struct Subscript {
    /// The type of this item
    r#type: MustBe!("Subscript"),

    /// The identifier for this item
    id: String,

    /// The content that is marked.
    content: Vec<Inline>,
}
