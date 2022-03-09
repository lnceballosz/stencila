use once_cell::sync::Lazy;
use provider::{
    codecs,
    eyre::{bail, eyre, Result},
    stencila_schema::Node,
    strum::VariantNames,
    tracing, EnrichOptions, ExportOptions, ImportOptions, ProviderTrait, SyncOptions,
};
use std::path::Path;
use std::sync::Arc;

pub use provider::{DetectItem, Provider};

// The following high level functions hide the implementation
// detail of having a static list of providers. They are intended as the
// only public interface for this crate.

pub async fn detect(node: &Node) -> Result<Vec<DetectItem>> {
    PROVIDERS.detect(node).await
}

pub async fn find(node: &Node) -> Result<Node> {
    let detections = detect(node).await?;
    let detection = match detections.len() {
        0 => bail!("No node detected"),
        1 => &detections[0],
        _ => {
            tracing::warn!("More than one node detected; will only use first");
            &detections[0]
        }
    };
    Ok(detection.node.clone())
}

pub async fn enrich(node: Node, options: Option<EnrichOptions>) -> Result<Node> {
    PROVIDERS.enrich(node, options).await
}

pub async fn import(node: &Node, path: &Path, options: Option<ImportOptions>) -> Result<bool> {
    PROVIDERS.import(node, path, options).await
}

pub async fn export(node: &Node, path: &Path, options: Option<ExportOptions>) -> Result<bool> {
    PROVIDERS.export(node, path, options).await
}

pub async fn sync(node: &Node, dest: &Path, options: Option<SyncOptions>) -> Result<bool> {
    PROVIDERS.sync(node, dest, options).await
}

pub async fn schedule(action: &str, schedule: &str, node: &Node, dest: &Path) -> Result<bool> {
    PROVIDERS.schedule(action, schedule, node, dest).await
}

/// The set of registered providers in the current process
static PROVIDERS: Lazy<Arc<Providers>> = Lazy::new(|| Arc::new(Providers::new()));

/// A set of registered providers, either built-in, or provided by plugins
struct Providers {
    inner: Vec<Provider>,
}

/// A macro to dispatch methods to builtin providers
macro_rules! dispatch_builtins {
    ($var:expr, $method:ident $(,$arg:expr)*) => {
        match $var.as_str() {
            #[cfg(feature = "provider-doi")]
            "doi" => Some(provider_doi::DoiProvider::$method($($arg),*)),
            #[cfg(feature = "provider-elife")]
            "elife" => Some(provider_elife::ElifeProvider::$method($($arg),*)),
            #[cfg(feature = "provider-gdrive")]
            "gdrive" => Some(provider_gdrive::GoogleDriveProvider::$method($($arg),*)),
            #[cfg(feature = "provider-github")]
            "github" => Some(provider_github::GithubProvider::$method($($arg),*)),
            #[cfg(feature = "provider-gitlab")]
            "gitlab" => Some(provider_gitlab::GitlabProvider::$method($($arg),*)),
            #[cfg(feature = "provider-http")]
            "http" => Some(provider_http::HttpProvider::$method($($arg),*)),
            _ => None
        }
    };
}

impl Providers {
    /// Create a set of providers
    ///
    /// Ordering is important because detection is done in order and often
    /// when there are multiple detections for the same location (e.g. a GitHub
    /// url and a generic HTTP/S url) the first is used.
    pub fn new() -> Self {
        let inner = vec![
            #[cfg(feature = "provider-doi")]
            provider_doi::DoiProvider::spec(),
            #[cfg(feature = "provider-elife")]
            provider_elife::ElifeProvider::spec(),
            #[cfg(feature = "provider-gdrive")]
            provider_gdrive::GoogleDriveProvider::spec(),
            #[cfg(feature = "provider-github")]
            provider_github::GithubProvider::spec(),
            #[cfg(feature = "provider-gitlab")]
            provider_gitlab::GitlabProvider::spec(),
            #[cfg(feature = "provider-http")]
            provider_http::HttpProvider::spec(),
        ];
        Self { inner }
    }

    /// List the available providers
    fn list(&self) -> Vec<String> {
        self.inner
            .iter()
            .map(|provider| provider.name.clone())
            .collect()
    }

    /// Get a provider by name
    fn get(&self, name: &str) -> Result<Provider> {
        for provider in &self.inner {
            if provider.name == name {
                return Ok(provider.clone());
            }
        }
        bail!("No provider with name `{}`", name)
    }

    /// Detect nodes within a node
    ///
    /// The `detect` method of each provider is called on the node and the result
    /// is a list of detections across all providers.
    async fn detect(&self, node: &Node) -> Result<Vec<DetectItem>> {
        let mut detected = Vec::new();
        for provider in &self.inner {
            if let Some(future) = dispatch_builtins!(provider.name, detect, node) {
                let mut result = future.await?;
                detected.append(&mut result);
            }
        }
        Ok(detected)
    }

    /// Enrich a node
    ///
    /// The `enrich` method of each provider is called on the node possibly mutating it with new
    /// and/or different values for fields.
    async fn enrich(&self, mut node: Node, options: Option<EnrichOptions>) -> Result<Node> {
        for provider in &self.inner {
            if let Some(future) =
                dispatch_builtins!(provider.name, enrich, node.clone(), options.clone())
            {
                node = future.await?;
            }
        }
        Ok(node)
    }

    /// Import content from a remote [`Node`] to a local path
    ///
    /// The `import` method of each provider is called until one returns `true` (indicating that the node was imported).
    /// If no providers are able to import the node returns `Ok(false)`.
    async fn import(
        &self,
        node: &Node,
        path: &Path,
        options: Option<ImportOptions>,
    ) -> Result<bool> {
        for provider in &self.inner {
            if let Some(future) =
                dispatch_builtins!(provider.name, import, node, path, options.clone())
            {
                let imported = future.await?;
                if imported {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    /// Export content from a local path to a remote [`Node`]
    ///
    /// The `export` method of each provider is called until one returns `true` (indicating that the node was exported).
    /// If no providers are able to export the node returns `Ok(false)`.
    async fn export(
        &self,
        node: &Node,
        path: &Path,
        options: Option<ExportOptions>,
    ) -> Result<bool> {
        for provider in &self.inner {
            if let Some(future) =
                dispatch_builtins!(provider.name, export, node, path, options.clone())
            {
                let exported = future.await?;
                if exported {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    /// Synchronize changes between a remote [`Node`] and a local path
    ///
    /// The `sync` method of each provider is called until one returns `true` (indicating that the node was synced).
    /// Ino providers are able to sync the node returns `Ok(false)`.
    async fn sync(&self, node: &Node, path: &Path, options: Option<SyncOptions>) -> Result<bool> {
        for provider in &self.inner {
            if let Some(future) =
                dispatch_builtins!(provider.name, sync, node, path, options.clone())
            {
                let syncing = future.await?;
                if syncing {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    /// Schedule import and/or export to/from a remove [`Node`] and a local path
    async fn schedule(
        &self,
        action: &str,
        schedule: &str,
        node: &Node,
        path: &Path,
    ) -> Result<bool> {
        for provider in &self.inner {
            if let Some(future) =
                dispatch_builtins!(provider.name, schedule, action, schedule, node, path)
            {
                let scheduleing = future.await?;
                if scheduleing {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
}

impl Default for Providers {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "cli")]
pub mod commands {
    use std::path::PathBuf;

    use super::*;
    use cli_utils::{async_trait::async_trait, result, Result, Run};
    use provider::SyncMode;
    use structopt::StructOpt;

    #[derive(StructOpt)]
    #[structopt(
        about = "Manage providers",
        setting = structopt::clap::AppSettings::ColoredHelp,
        setting = structopt::clap::AppSettings::DeriveDisplayOrder,
        setting = structopt::clap::AppSettings::VersionlessSubcommands
    )]
    pub enum Command {
        List(List),
        Show(Show),
        Detect(Detect),
        Enrich(Enrich),
        Import(Import),
        Export(Export),
        Sync(Sync),
        Schedule(Schedule),
    }

    #[async_trait]
    impl Run for Command {
        async fn run(&self) -> Result {
            match self {
                Command::List(action) => action.run().await,
                Command::Show(action) => action.run().await,
                Command::Detect(action) => action.run().await,
                Command::Enrich(action) => action.run().await,
                Command::Import(action) => action.run().await,
                Command::Export(action) => action.run().await,
                Command::Sync(action) => action.run().await,
                Command::Schedule(action) => action.run().await,
            }
        }
    }

    /// List the providers that are available
    ///
    /// The list of available providers includes those that are built into the Stencila
    /// binary as well as those provided by plugins.
    #[derive(StructOpt)]
    #[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
    pub struct List {}
    #[async_trait]
    impl Run for List {
        async fn run(&self) -> Result {
            let list = PROVIDERS.list();
            result::value(list)
        }
    }

    /// Show the specifications of a provider
    #[derive(StructOpt)]
    #[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
    pub struct Show {
        /// The name of the provider
        ///
        /// To get the list of provider names using `stencila providers list`.
        name: String,
    }
    #[async_trait]
    impl Run for Show {
        async fn run(&self) -> Result {
            let provider = PROVIDERS.get(&self.name)?;
            result::value(provider)
        }
    }

    /// Detect nodes within a file or string
    #[derive(StructOpt)]
    #[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
    pub struct Detect {
        /// The path to the file (or the string value if the `--string` flag is used)
        path: PathBuf,

        /// The format of the file; defaults to the file extension
        format: Option<String>,

        /// If the argument should be treated as a string, rather than a file path
        #[structopt(short, long)]
        string: bool,
    }
    #[async_trait]
    impl Run for Detect {
        async fn run(&self) -> Result {
            let node = if self.string {
                let string = self
                    .path
                    .to_str()
                    .ok_or_else(|| eyre!("Value is not valid Unicode"))?
                    .into();
                Node::String(string)
            } else {
                codecs::from_path(&self.path, self.format.as_deref(), None).await?
            };

            let detections = detect(&node).await?;
            result::value(detections)
        }
    }

    /// Enrich nodes within a file or string
    #[derive(StructOpt)]
    #[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
    pub struct Enrich {
        /// The path to the file (or the string value if the `--string` flag is used)
        path: PathBuf,

        /// The format of the file; defaults to the file extension
        format: Option<String>,

        /// If the argument should be treated as a string, rather than a file path
        #[structopt(short, long)]
        string: bool,

        /// Any access token required by the source provider
        #[structopt(long, short)]
        token: Option<String>,
    }
    #[async_trait]
    impl Run for Enrich {
        async fn run(&self) -> Result {
            let node = if self.string {
                let string = self
                    .path
                    .to_str()
                    .ok_or_else(|| eyre!("Value is not valid Unicode"))?
                    .into();
                Node::String(string)
            } else {
                codecs::from_path(&self.path, self.format.as_deref(), None).await?
            };

            let detections = detect(&node).await?;

            let mut nodes: Vec<Node> = Vec::with_capacity(detections.len());
            let options = EnrichOptions {
                token: self.token.clone(),
            };
            for detection in detections.into_iter() {
                let node = enrich(detection.node, Some(options.clone())).await?;
                nodes.push(node);
            }

            result::value(nodes)
        }
    }

    /// Import content from a remote source to a local path
    #[derive(StructOpt)]
    #[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
    pub struct Import {
        /// The source identifier e.g. `github:org/name@v1.2.0`
        source: String,

        /// The local path to import file/s to e.g. `data`
        #[structopt(default_value = ".")]
        path: PathBuf,

        /// Any access token required by the source provider
        #[structopt(long, short)]
        token: Option<String>,
    }
    #[async_trait]
    impl Run for Import {
        async fn run(&self) -> Result {
            let identifier = Node::String(self.source.clone());
            let node = find(&identifier).await?;

            let options = ImportOptions {
                token: self.token.clone(),
            };
            let imported = import(&node, &self.path, Some(options)).await?;
            if !imported {
                tracing::error!("Unable to import from source `{}`", self.source);
            }

            result::nothing()
        }
    }

    /// Export content from a local path to a remote source
    #[derive(StructOpt)]
    #[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
    pub struct Export {
        /// The source identifier e.g. `github:org/name@v1.2.0`
        source: String,

        /// The local path to export file/s from e.g. `report.md`
        #[structopt(default_value = ".")]
        path: PathBuf,

        /// Any access token required by the source provider
        #[structopt(long, short)]
        token: Option<String>,
    }
    #[async_trait]
    impl Run for Export {
        async fn run(&self) -> Result {
            let identifier = Node::String(self.source.clone());
            let node = find(&identifier).await?;

            let options = ExportOptions {
                token: self.token.clone(),
            };
            let exported = export(&node, &self.path, Some(options)).await?;
            if !exported {
                tracing::error!("Unable to export to source `{}`", self.source);
            }

            result::nothing()
        }
    }

    /// Synchronize changes between a remote source and a local path
    #[derive(StructOpt)]
    #[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
    pub struct Sync {
        /// The source identifier e.g. `github:org/name`
        source: String,

        /// The local path to synchronize with the source
        #[structopt(default_value = ".")]
        path: PathBuf,

        /// The synchronization mode
        #[structopt(long, short, possible_values = &SyncMode::VARIANTS)]
        mode: Option<SyncMode>,

        /// Any access token required by the source provider
        #[structopt(long, short)]
        token: Option<String>,

        /// The URL to listen on for events from the source provider
        #[structopt(long, short)]
        url: Option<String>,
    }
    #[async_trait]
    impl Run for Sync {
        async fn run(&self) -> Result {
            let identifier = Node::String(self.source.clone());
            let node = find(&identifier).await?;

            let options = SyncOptions {
                mode: self.mode.clone(),
                token: self.token.clone(),
                url: self.url.clone(),
            };
            let syncing = sync(&node, &self.path, Some(options)).await?;
            if !syncing {
                tracing::error!("Unable to synchronize with source `{}`", self.source);
            }

            result::nothing()
        }
    }

    /// Schedule changes between a remote source and a local path
    #[derive(StructOpt)]
    #[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
    pub struct Schedule {
        /// The action to take at the scheduled time
        #[structopt(possible_values=provider::ACTIONS)]
        action: String,

        /// The schedule on which to perform the action
        schedule: String,

        /// The source identifier e.g. `github:org/name`
        source: String,

        /// The local path to synchronize with the source
        #[structopt(default_value = ".")]
        path: PathBuf,
    }
    #[async_trait]
    impl Run for Schedule {
        async fn run(&self) -> Result {
            let identifier = Node::String(self.source.clone());
            let node = find(&identifier).await?;

            let scheduling = schedule(&self.action, &self.schedule, &node, &self.path).await?;
            if !scheduling {
                tracing::error!(
                    "Unable to schedule `{}` of source `{}`",
                    self.action,
                    self.source
                );
            }

            result::nothing()
        }
    }
}
