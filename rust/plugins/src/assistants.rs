use std::sync::Arc;

use assistant::{choose_model, deserialize_models, serialize_models};
use common::{
    async_trait::async_trait,
    eyre::{bail, Result},
    inflector::Inflector,
    serde::{Deserialize, Serialize},
    tokio::sync::Mutex,
};
use kernel::schema::AuthorRoleName;
use model::{
    format::Format, GenerateOptions, GenerateOutput, GenerateTask, Model, ModelAvailability,
    ModelIO, ModelType,
};

use crate::{plugins, Plugin, PluginEnabled, PluginInstance, PluginStatus};

/// A assistant provided by a plugin
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "common::serde")]
pub struct PluginAssistant {
    /// The name of the assistant
    name: String,

    /// The name of the assistant
    ///
    /// Will be extracted from the name if not supplied
    title: Option<String>,

    /// A description of the assistant
    description: Option<String>,

    /// The input types that the assistant supports
    #[serde(default)]
    inputs: Vec<ModelIO>,

    /// The output types that the assistant supports
    #[serde(default)]
    outputs: Vec<ModelIO>,

    /// The format that the content of the instruction should
    /// be formatted using for use in the system prompt
    #[serde(alias = "content-format")]
    content_format: Option<Format>,

    /// Whether the assistant provides a system prompt
    ///
    /// If this is `true` then the assistant will be requested to provide a system prompt
    /// string for each instruction. The string can be dynamically generated by the assistant
    /// but will also be treated as a Jinja template and rendered by Stencila given the
    /// document context.
    #[serde(alias = "system-prompt")]
    system_prompt: bool,

    /// The names of the assistants this assistant will model
    /// to in descending order of preference
    ///
    /// The default ordered list of models can be prepended
    /// using this options. If the last item is `only` then the
    /// list will be limited to those specified.
    #[serde(
        deserialize_with = "deserialize_models",
        serialize_with = "serialize_models"
    )]
    models: Vec<String>,

    /// The plugin that provides this assistant
    ///
    /// Used to be able to create a plugin instance, which in
    /// turn is used to create a assistant instance.
    #[serde(skip)]
    plugin: Option<Plugin>,

    /// The plugin instance for this assistant. Used to avoid starting
    /// a new instance for each call to the assistant.
    ///
    /// This needs to be a `Arc<Mutex>` because the `perform_task` method is async
    /// but is not `&mut self`. As such, this is needed for "interior mutability" across
    /// calls to that method.
    #[serde(skip)]
    plugin_instance: Arc<Mutex<Option<PluginInstance>>>,
}

impl PluginAssistant {
    /// Bind a plugin to this assistant so that it can be started (by starting the plugin first)
    pub fn bind(&mut self, plugin: &Plugin) {
        self.plugin = Some(plugin.clone());
    }
}

#[async_trait]
impl Model for PluginAssistant {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn r#type(&self) -> ModelType {
        match &self.plugin {
            Some(plugin) => {
                let mut name = plugin.name.clone();
                if plugin.linked {
                    name += " (linked)";
                }
                ModelType::Plugin(name)
            }
            None => ModelType::Plugin("unknown".to_string()),
        }
    }

    fn availability(&self) -> ModelAvailability {
        match &self.plugin {
            Some(plugin) => match plugin.availability() {
                (
                    PluginStatus::InstalledLatest(..) | PluginStatus::InstalledOutdated(..),
                    PluginEnabled::Yes,
                ) => ModelAvailability::Available,

                (
                    PluginStatus::InstalledLatest(..) | PluginStatus::InstalledOutdated(..),
                    PluginEnabled::No,
                ) => ModelAvailability::Disabled,

                (PluginStatus::Installable, _) => ModelAvailability::Installable,

                _ => ModelAvailability::Unavailable,
            },
            None => ModelAvailability::Unavailable,
        }
    }

    fn title(&self) -> String {
        self.title.clone().unwrap_or_else(|| {
            let id = self.name.clone();
            let name = id
                .rsplit_once('/')
                .map(|(.., name)| name.split_once('-').map_or(name, |(name, ..)| name))
                .unwrap_or(&id);
            name.to_title_case()
        })
    }

    fn version(&self) -> String {
        self.plugin
            .as_ref()
            .map(|plugin| plugin.version.to_string())
            .unwrap_or_default()
    }

    fn supported_inputs(&self) -> &[ModelIO] {
        if self.inputs.is_empty() {
            &[ModelIO::Text]
        } else {
            &self.inputs
        }
    }

    fn supported_outputs(&self) -> &[ModelIO] {
        if self.outputs.is_empty() {
            &[ModelIO::Text]
        } else {
            &self.outputs
        }
    }

    async fn perform_task(
        &self,
        task: &GenerateTask,
        options: &GenerateOptions,
    ) -> Result<GenerateOutput> {
        // Create the plugin instance if necessary
        let mut guard = self.plugin_instance.lock().await;
        let instance = match &mut *guard {
            Some(instance) => instance,
            None => {
                let Some(plugin) = self.plugin.as_ref() else {
                    bail!("Not bound yet")
                };

                let inst = plugin.start(None).await?;
                *guard = Some(inst);
                guard.as_mut().unwrap()
            }
        };

        // If the assistant provides a system prompt then make a request for it
        // providing the task and options in case the assistant wants to use those for generating
        // the system prompt.
        // Also create a prompter author rile with timestamp now, before generation.
        let (system_prompt, prompter_role) = if self.system_prompt {
            // Call the plugin assistant's `system_prompt` method
            #[derive(Serialize)]
            #[serde(crate = "common::serde")]
            struct Params {
                assistant: String,
                task: GenerateTask,
                options: GenerateOptions,
            }
            let system_prompt: String = instance
                .call(
                    "assistant_system_prompt",
                    Params {
                        assistant: self.name(),
                        task: task.clone(),
                        options: options.clone(),
                    },
                )
                .await?;

            let prompter_role = self.to_author_role(AuthorRoleName::Prompter);

            (Some(system_prompt), Some(prompter_role))
        } else {
            (None, None)
        };

        // If the assistant has models choose one for the task
        let model = if self.models.is_empty() {
            None
        } else {
            Some(choose_model(&self.models, task).await?)
        };

        // Prepare the task for the model (if any) including
        // formatting the content and rendering the system prompt
        let mut task = task.clone();
        if self.content_format.is_some() || system_prompt.is_some() {
            task.prepare(
                model.as_deref(),
                self.content_format.as_ref(),
                system_prompt.as_ref(),
            )
            .await?;
        }

        let output: GenerateOutput = if let Some(model) = model {
            // Get model to perform the task
            model.perform_task(&task, options).await?
        } else {
            // Call the plugin assistant's `perform_task` method
            #[derive(Serialize)]
            #[serde(crate = "common::serde")]
            struct Params {
                assistant: String,
                task: GenerateTask,
                options: GenerateOptions,
            }
            instance
                .call(
                    "assistant_perform_task",
                    Params {
                        assistant: self.name(),
                        task: task.clone(),
                        options: options.clone(),
                    },
                )
                .await?
        };

        // Post process the output
        let format = if output.format == Format::Unknown {
            task.format().clone()
        } else {
            output.format.clone()
        };
        let mut output =
            GenerateOutput::from_plugin(output, self, &format, task.instruction(), options).await?;

        // Add the prompter role, if any. Intentionally appended, not prepended, so that
        // the generator is the primary author
        if let Some(prompter_role) = prompter_role {
            output.authors.push(prompter_role);
        }

        Ok(output)
    }
}

/// List all the assistants provided by plugins
pub async fn list() -> Result<Vec<Arc<dyn Model>>> {
    Ok(plugins()
        .await
        .into_iter()
        .flat_map(|plugin| plugin.assistants())
        .collect())
}
