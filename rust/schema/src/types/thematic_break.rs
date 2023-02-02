//! Generated file, do not edit

use crate::prelude::*;

use super::string::String;

/// A thematic break, such as a scene change in a story, a transition to another topic, or a new document.
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "common::serde")]
pub struct ThematicBreak {
    /// The type of this item
    r#type: MustBe!("ThematicBreak"),

    /// The identifier for this item
    id: String,
}
