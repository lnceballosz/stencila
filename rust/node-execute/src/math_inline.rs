use schema::MathInline;

use crate::prelude::*;

impl Executable for MathInline {
    #[tracing::instrument(skip_all)]
    async fn compile(&mut self, executor: &mut Executor) -> WalkControl {
        let node_id = self.node_id();

        let compilation_digest = parsers::parse(
            &self.code,
            self.math_language.as_deref().unwrap_or_default(),
        )
        .compilation_digest;

        if Some(&compilation_digest) == self.options.compilation_digest.as_ref() {
            tracing::trace!("Skipping compiling MathInline {node_id}");

            return WalkControl::Break;
        }

        tracing::trace!("Compiling MathInline {node_id}");

        let code = self.code.trim();
        if !code.is_empty() {
            let lang = self
                .math_language
                .as_ref()
                .map_or("tex".to_string(), |lang| lang.to_lowercase());

            let (mathml, messages) = if lang == "mathml" {
                (Node::String(code.to_string()), None)
            } else {
                let (mathml, messages) = executor
                    .kernels()
                    .await
                    .evaluate(code, Some(&lang))
                    .await
                    .unwrap_or_else(|error| {
                        (
                            Node::String(String::new()),
                            vec![error_to_message("While compiling math", error)],
                        )
                    });

                let messages = (!messages.is_empty()).then_some(messages);

                (mathml, messages)
            };

            executor.replace_properties(
                &node_id,
                [
                    (Property::MathMl, mathml.into()),
                    (Property::CompilationMessages, messages.into()),
                    (Property::CompilationDigest, compilation_digest.into()),
                ],
            );
        } else {
            executor.replace_properties(
                &node_id,
                [
                    (Property::MathMl, Value::None),
                    (Property::CompilationMessages, Value::None),
                    (Property::CompilationDigest, compilation_digest.into()),
                ],
            );
        };

        WalkControl::Continue
    }
}
