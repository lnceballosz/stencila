// Generated file; do not edit. See `schema-gen` crate.

use crate::prelude::*;

/// A `roleName` for an `AuthorRole`.
#[derive(Debug, strum::Display, Clone, PartialEq, Serialize, Deserialize, StripNode, WalkNode, WriteNode, SmartDefault, strum::EnumString, Eq, PartialOrd, Ord, ReadNode, PatchNode, DomCodec, HtmlCodec, JatsCodec, MarkdownCodec, TextCodec)]
#[serde(crate = "common::serde")]
#[strum(ascii_case_insensitive, crate = "common::strum")]
pub enum AuthorRoleName {
    /// The author, usually a `Person`, imported content from an external format into a new document.
    Importer,

    /// The author, usually a `Person`, wrote content including inserting, deleting and replacing prose and code.
    #[default]
    Writer,

    /// The author, usually a `Person`, verified the accuracy of content, usually generated by a `SoftwareApplication`.
    Verifier,

    /// The author, usually a `Person`, instructed another author, usually a `SoftwareApplication`, to create content.
    Instructor,

    /// The author, usually a `SoftwareApplication`, prompted another author, also usually a `SoftwareApplication`, to generate content.
    Prompter,

    /// The author is a `SoftwareApplication` that generated content such as prose, code, or images.
    Generator,
}
