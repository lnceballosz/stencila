use codec::{
    common::{async_trait::async_trait, eyre::Result},
    format::Format,
    schema::{Node, NodeType},
    status::Status,
    Codec, CodecSupport, DecodeOptions, EncodeOptions, LossDirection, Losses,
};

mod decode;
mod encode;

/// A codec for JATS
pub struct JatsCodec;

#[async_trait]
impl Codec for JatsCodec {
    fn name(&self) -> &str {
        "jats"
    }

    fn status(&self) -> Status {
        Status::UnderDevelopment
    }

    fn supports_from_format(&self, format: Format) -> CodecSupport {
        match format {
            Format::Jats => CodecSupport::LowLoss,
            _ => CodecSupport::None,
        }
    }

    fn supports_to_format(&self, format: Format) -> CodecSupport {
        match format {
            Format::Jats => CodecSupport::LowLoss,
            _ => CodecSupport::None,
        }
    }

    fn supports_from_type(&self, node_type: NodeType) -> CodecSupport {
        use CodecSupport::*;
        use NodeType::*;
        match node_type {
            // Prose Inlines
            Text | Emphasis | Strong | Strikeout | Subscript | Superscript | Underline | Insert => {
                NoLoss
            }
            // Prose Blocks
            Paragraph | ThematicBreak => NoLoss,
            // Works,
            Article => LowLoss,
            _ => None,
        }
    }

    fn supports_to_type(&self, node_type: NodeType) -> CodecSupport {
        use CodecSupport::*;
        use NodeType::*;
        match node_type {
            // Data
            String | Cord => NoLoss,
            Null | Boolean | Integer | UnsignedInteger | Number => LowLoss,
            // Prose Inlines
            Text | Emphasis | Strong | Strikeout | Subscript | Superscript | Underline | Insert => {
                NoLoss
            }
            Link | Parameter | AudioObject | ImageObject | MediaObject => LowLoss,
            // Prose Blocks
            Heading | Paragraph | ThematicBreak => NoLoss,
            List | ListItem | Table | TableRow | TableCell => LowLoss,
            // Code
            CodeFragment | CodeBlock => NoLoss,
            CodeExpression | CodeChunk => LowLoss,
            // Math
            MathFragment | MathBlock => NoLoss,
            // Works,
            Article => LowLoss,
            // If not in the above lists then no support
            _ => None,
        }
    }

    async fn from_str(&self, str: &str, options: Option<DecodeOptions>) -> Result<(Node, Losses)> {
        decode::decode(str, options)
    }

    async fn to_string(
        &self,
        node: &Node,
        options: Option<EncodeOptions>,
    ) -> Result<(String, Losses)> {
        encode::encode(node, options)
    }
}
