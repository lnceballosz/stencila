use common::{
    eyre::Result,
    serde::{de::DeserializeOwned, Serialize},
    serde_json,
};

pub trait FromJson: DeserializeOwned {
    /// Deserialize a node from JSON
    fn from_json(json: &str) -> Result<Self> {
        Ok(serde_json::from_str(json)?)
    }

    /// Deserialize a node from a [`serde_json::Value`]
    fn from_json_value(json: serde_json::Value) -> Result<Self> {
        Ok(serde_json::from_value::<Self>(json)?)
    }
}

impl<T> FromJson for T where T: DeserializeOwned {}

pub trait ToJson: Serialize {
    /// Serialize a node to JSON
    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }

    /// Serialize a node to indented JSON
    fn to_json_pretty(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// Serialize a node to a [`serde_json::Value`]
    fn to_json_value(&self) -> Result<serde_json::Value> {
        Ok(serde_json::to_value(self)?)
    }
}

impl<T> ToJson for T where T: Serialize {}

#[cfg(test)]
mod tests {
    //! Most of these tests are trivial but are important given that we use
    //! them as the main unit tests for `serde` settings (e.g. untagged, flatten)
    //! and other things (e.g. ordering of types in the `Node` enum) which
    //! affect serialization and deserialization in the `schema` crate.
    //!
    //! Other `serde`-based codecs (e.g. `yaml`) do not have as comprehensive unit tests
    //! (although they do have round-trip prop tests) because they should work if these tests pass).

    use std::collections::HashMap;

    use common_dev::pretty_assertions::assert_eq;
    use schema::{
        Array, Article, ArticleOptions, Block, Boolean, Date, Emphasis, Inline, Integer, Node,
        Null, Number, Object, Paragraph, Primitive, Time,
    };

    use super::*;

    /// Test deserialization of primitive types from JSON
    #[test]
    fn primitive_types_from_json() -> Result<()> {
        assert_eq!(Null::from_json("null")?, Null {});

        assert_eq!(Boolean::from_json("true")?, true);
        assert_eq!(Boolean::from_json("false")?, false);

        assert_eq!(Integer::from_json("123")?, 123);
        assert_eq!(Integer::from_json("-123")?, -123);

        assert_eq!(Number::from_json("1.23")?, 1.23);
        assert_eq!(Number::from_json("-1.23")?, -1.23);

        assert_eq!(String::from_json(r#""""#)?, String::default());
        assert_eq!(
            String::from_json("\"Hello world\"")?,
            "Hello world".to_string()
        );

        assert_eq!(Array::from_json("[]")?, vec![]);
        assert_eq!(
            Array::from_json(r#"[false, 1, 1.23, "abc"]"#)?,
            vec![
                Primitive::Boolean(false),
                Primitive::Integer(1),
                Primitive::Number(1.23),
                Primitive::String("abc".to_string())
            ]
        );

        assert_eq!(Object::from_json("{}")?, Object::default());
        assert_eq!(
            Object::from_json(r#"{"a": 1, "b": [], "c": {"d": true}}"#)?,
            HashMap::from([
                ("a".to_string(), Primitive::Integer(1)),
                ("b".to_string(), Primitive::Array(vec![])),
                (
                    "c".to_string(),
                    Primitive::Object(HashMap::from([("d".to_string(), Primitive::Boolean(true))]))
                )
            ])
        );

        Ok(())
    }

    /// Test deserialization of `Primitive` enum from JSON
    #[test]
    fn primitive_enum_from_json() -> Result<()> {
        assert_eq!(Primitive::from_json("null")?, Primitive::Null(Null {}));
        assert_eq!(Primitive::from_json("true")?, Primitive::Boolean(true));
        assert_eq!(Primitive::from_json("123")?, Primitive::Integer(123));
        assert_eq!(Primitive::from_json("1.23")?, Primitive::Number(1.23));
        assert_eq!(
            Primitive::from_json(r#""abc""#)?,
            Primitive::String("abc".to_string())
        );
        assert_eq!(
            Primitive::from_json("[]")?,
            Primitive::Array(Vec::default())
        );
        assert_eq!(
            Primitive::from_json("{}")?,
            Primitive::Object(HashMap::default())
        );

        Ok(())
    }

    /// Test deserialization of various entity types, including those with `options`
    #[test]
    fn entity_types_from_json() -> Result<()> {
        assert_eq!(
            Date::from_json(r#"{ "type":"Date", "value": "2022-02-02" }"#)?,
            Date {
                value: "2022-02-02".to_string(),
                ..Default::default()
            }
        );

        assert_eq!(
            Article::from_json(
                r#"{
                    "type": "Article",
                    "content": [
                        {
                            "type": "Paragraph",
                            "content": ["Hello world"]
                        }
                    ],
                    "pageStart": 1,
                    "pageEnd": "MXC"
                }"#
            )?,
            Article {
                content: vec![Block::Paragraph(Paragraph {
                    content: vec![Inline::String("Hello world".to_string())],
                    ..Default::default()
                })],
                options: Box::new(ArticleOptions {
                    page_start: Some(schema::IntegerOrString::Integer(1)),
                    page_end: Some(schema::IntegerOrString::String("MXC".to_string())),
                    ..Default::default()
                }),
                ..Default::default()
            }
        );

        Ok(())
    }

    /// Test deserialization of various entity enums from JSON
    #[test]
    fn entity_enum_from_json() -> Result<()> {
        assert_eq!(
            Inline::from_json(r#""abc""#)?,
            Inline::String("abc".to_string())
        );

        assert_eq!(
            Inline::from_json(r#"{ "type":"Emphasis", "content":[] }"#)?,
            Inline::Emphasis(Emphasis {
                content: vec![],
                ..Default::default()
            })
        );

        assert_eq!(
            Block::from_json(r#"{ "type":"Paragraph", "content":[] }"#)?,
            Block::Paragraph(Paragraph {
                content: vec![],
                ..Default::default()
            })
        );

        assert_eq!(Node::from_json("123")?, Node::Integer(123));

        assert_eq!(
            Node::from_json(r#""abc""#)?,
            Node::String("abc".to_string())
        );

        assert_eq!(
            Node::from_json(r#"{ "type":"Time", "value":"01:02:03" }"#)?,
            Node::Time(Time {
                value: "01:02:03".to_string(),
                ..Default::default()
            })
        );

        Ok(())
    }
}
