//! Generated file, do not edit

use crate::prelude::*;

use super::inline::Inline;
use super::integer::Integer;
use super::string::String;

/// A heading.
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "common::serde")]
pub struct Heading {
    /// The type of this item
    r#type: MustBe!("Heading"),

    /// The identifier for this item
    id: String,

    /// The depth of the heading.
    #[def = "1"]
    depth: Integer,

    /// Content of the heading.
    content: Vec<Inline>,
}
