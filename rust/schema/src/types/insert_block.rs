// Generated file; do not edit. See `schema-gen` crate.

use crate::prelude::*;

use super::author::Author;
use super::block::Block;
use super::duration::Duration;
use super::provenance_count::ProvenanceCount;
use super::string::String;
use super::suggestion_status::SuggestionStatus;
use super::timestamp::Timestamp;

/// A suggestion to insert some block content.
#[skip_serializing_none]
#[serde_as]
#[derive(Debug, SmartDefault, Clone, PartialEq, Serialize, Deserialize, StripNode, WalkNode, WriteNode, ReadNode, PatchNode, DomCodec, HtmlCodec, JatsCodec, TextCodec)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
#[cfg_attr(feature = "proptest", derive(Arbitrary))]
#[derive(derive_more::Display)]
#[display(fmt = "InsertBlock")]
#[patch(authors_on = "self")]
#[jats(special)]
pub struct InsertBlock {
    /// The type of this item.
    #[cfg_attr(feature = "proptest", proptest(value = "Default::default()"))]
    pub r#type: MustBe!("InsertBlock"),

    /// The identifier for this item.
    #[strip(metadata)]
    #[cfg_attr(feature = "proptest", proptest(value = "None"))]
    #[html(attr = "id")]
    pub id: Option<String>,

    /// The status of the suggestion including whether it is proposed, accepted, or rejected.
    #[serde(alias = "suggestion-status", alias = "suggestion_status")]
    #[strip(metadata)]
    #[patch(format = "md", format = "myst")]
    #[cfg_attr(feature = "proptest", proptest(value = "None"))]
    pub suggestion_status: Option<SuggestionStatus>,

    /// The authors of the suggestion
    #[serde(alias = "author")]
    #[serde(default, deserialize_with = "option_one_or_many_string_or_object")]
    #[strip(authors)]
    #[cfg_attr(feature = "proptest", proptest(value = "None"))]
    #[dom(elem = "span")]
    pub authors: Option<Vec<Author>>,

    /// A summary of the provenance of the content within the suggestion.
    #[serde(default, deserialize_with = "option_one_or_many")]
    #[strip(provenance)]
    #[cfg_attr(feature = "proptest", proptest(value = "None"))]
    #[dom(elem = "span")]
    pub provenance: Option<Vec<ProvenanceCount>>,

    /// Time taken to generate the suggestion.
    #[serde(alias = "execution-duration", alias = "execution_duration")]
    #[strip(execution)]
    #[cfg_attr(feature = "proptest", proptest(value = "None"))]
    #[dom(with = "Duration::to_dom_attr")]
    pub execution_duration: Option<Duration>,

    /// The timestamp when the generation ended.
    #[serde(alias = "execution-ended", alias = "execution_ended")]
    #[strip(execution, timestamps)]
    #[cfg_attr(feature = "proptest", proptest(value = "None"))]
    #[dom(with = "Timestamp::to_dom_attr")]
    pub execution_ended: Option<Timestamp>,

    /// Feedback on the suggestion
    #[patch(format = "md", format = "myst")]
    #[cfg_attr(feature = "proptest", proptest(value = "None"))]
    pub feedback: Option<String>,

    /// The content that is suggested to be inserted, modified, replaced, or deleted.
    #[serde(deserialize_with = "one_or_many")]
    #[walk]
    #[patch(format = "all")]
    #[cfg_attr(feature = "proptest-min", proptest(value = r#"vec![p([t("text")])]"#))]
    #[cfg_attr(feature = "proptest-low", proptest(strategy = r#"vec_blocks_non_recursive(1)"#))]
    #[cfg_attr(feature = "proptest-high", proptest(strategy = r#"vec_blocks_non_recursive(2)"#))]
    #[cfg_attr(feature = "proptest-max", proptest(strategy = r#"vec_blocks_non_recursive(4)"#))]
    #[dom(elem = "div")]
    pub content: Vec<Block>,

    /// A unique identifier for a node within a document
    #[cfg_attr(feature = "proptest", proptest(value = "Default::default()"))]
    #[serde(skip)]
    pub uid: NodeUid
}

impl InsertBlock {
    const NICK: [u8; 3] = [105, 110, 98];
    
    pub fn node_type(&self) -> NodeType {
        NodeType::InsertBlock
    }

    pub fn node_id(&self) -> NodeId {
        NodeId::new(&Self::NICK, &self.uid)
    }
    
    pub fn new(content: Vec<Block>) -> Self {
        Self {
            content,
            ..Default::default()
        }
    }
}
