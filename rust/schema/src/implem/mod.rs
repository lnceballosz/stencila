//! Implementations of traits for types

mod admonition;
mod array;
mod article;
mod author;
mod author_role;
mod automatic_execution;
mod block;
mod call_argument;
mod call_block;
mod claim;
mod code_block;
mod code_chunk;
mod code_expression;
mod code_inline;
mod cord;
mod datatable;
mod datatable_columns;
mod date;
mod date_time;
mod delete_block;
mod duration;
mod execution_status;
mod figure;
mod for_block;
mod heading;
mod if_block;
mod if_block_clause;
mod include_block;
mod inline;
mod insert_block;
mod insert_inline;
mod instruction_block;
mod instruction_inline;
mod instruction_message;
mod integer_or_string;
mod link;
mod list;
mod list_item;
mod math_block;
mod math_inline;
mod media_objects;
mod message_part;
mod modify_block;
mod modify_inline;
mod modify_operation;
mod node;
mod note;
mod null;
mod object;
mod parameter;
mod person;
mod person_or_organization;
mod primitive;
mod property_value_or_string;
mod quote_block;
mod replace_block;
mod section;
mod string_or_number;
mod string_patch;
mod string_patch_or_primitive;
mod styled_block;
mod styled_inline;
mod table;
mod text;
mod time;
mod timestamp;
mod validators;

pub use author::AuthorType;
