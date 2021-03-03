use crate::util;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub serve: crate::serve::config::Config,

    #[serde(default)]
    pub upgrade: crate::upgrade::config::Config,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            serve: Default::default(),
            upgrade: Default::default(),
        }
    }
}

const CONFIG_FILE: &str = "config.toml";

/// Get the path of the configuration file
fn path() -> Result<PathBuf> {
    #[cfg(not(test))]
    return Ok(util::dirs::config(true)?.join(CONFIG_FILE));

    // When running tests, avoid messing with users existing config
    #[cfg(test)]
    {
        let path = std::env::temp_dir()
            .join("stencila")
            .join("test")
            .join(CONFIG_FILE);
        fs::create_dir_all(path.parent().unwrap())?;
        return Ok(path);
    }
}

/// Read the config from the configuration file
fn read() -> Result<Config> {
    let config_file = path()?;
    let content = fs::read_to_string(config_file).unwrap_or_else(|_| "".to_string());
    let config = toml::from_str(content.as_str())?;
    Ok(config)
}

/// Write the config to the configuration file
fn write(config: Config) -> Result<()> {
    let config_file = path()?;
    let mut file = fs::File::create(config_file)?;
    file.write_all(toml::to_string(&config)?.as_bytes())?;
    file.sync_data()?;
    Ok(())
}

/// Get the config
pub fn get() -> Result<Config> {
    // Currently this just calls read but in future may
    // use a stored config object to avoid multiple reads
    read()
}

/// Ensure that a string is a valid JSON pointer
///
/// Replaces dots (`.`) with slashes (`/`) and ensures a
/// leading slash.
pub fn json_pointer(pointer: &str) -> String {
    let pointer = pointer.replace(".", "/");
    if pointer.starts_with('/') {
        pointer
    } else {
        format!("/{}", pointer)
    }
}

/// Display a config property
pub fn display(pointer: Option<String>) -> Result<String> {
    let config = get()?;
    match pointer {
        None => Ok(toml::to_string(&config)?),
        Some(pointer) => {
            let config = serde_json::to_value(config)?;
            if let Some(part) = config.pointer(json_pointer(&pointer).as_str()) {
                let json = serde_json::to_string(part)?;
                let part: toml::Value = serde_json::from_str(&json)?;
                let toml = toml::to_string(&part)?;
                Ok(toml)
            } else {
                bail!("No configuration value at pointer: {}", pointer)
            }
        }
    }
}

/// Set a config property
pub fn set(pointer: String, value: String) -> Result<()> {
    let config = read()?;

    let mut config = serde_json::to_value(config)?;
    if let Some(property) = config.pointer_mut(json_pointer(&pointer).as_str()) {
        let value = match serde_json::from_str(&value) {
            Ok(value) => value,
            Err(_) => serde_json::Value::String(value),
        };
        *property = value;
    } else {
        bail!("No configuration value at pointer: {}", pointer)
    };

    let config: Config = serde_json::from_value(config)?;
    write(config)
}

/// Reset a config property
pub fn reset(property: String) -> Result<()> {
    let config = read()?;

    let config: Config = match property.as_str() {
        "all" => Default::default(),
        "serve" => Config {
            serve: Default::default(),
            ..config
        },
        "upgrade" => Config {
            upgrade: Default::default(),
            ..config
        },
        _ => bail!("No configuration property named: {}", property),
    };

    write(config)
}

/// CLI options for the `config` command
#[cfg(feature = "cli")]
pub mod cli {
    use super::*;
    use structopt::StructOpt;

    #[derive(Debug, StructOpt)]
    #[structopt(
        about = "Manage configuration options",
        setting = structopt::clap::AppSettings::DeriveDisplayOrder
    )]
    pub struct Args {
        #[structopt(subcommand)]
        pub action: Action,
    }

    #[derive(Debug, StructOpt)]
    #[structopt(
        setting = structopt::clap::AppSettings::DeriveDisplayOrder
    )]
    pub enum Action {
        Get(Get),
        Set(Set),
        Reset(Reset),

        #[structopt(about = "Get the directories used for config, cache etc")]
        Dirs,
    }

    #[derive(Debug, StructOpt)]
    #[structopt(about = "Get configuration properties")]
    pub struct Get {
        /// A pointer to a config property e.g. `upgrade.auto`
        pub pointer: Option<String>,
    }

    #[derive(Debug, StructOpt)]
    #[structopt(
        about = "Set configuration properties",
        setting = structopt::clap::AppSettings::TrailingVarArg,
        setting = structopt::clap::AppSettings::AllowLeadingHyphen
    )]
    pub struct Set {
        /// A pointer to a config property e.g. `upgrade.auto`
        pub pointer: String,

        /// The value to set the property to
        pub value: String,
    }

    #[derive(Debug, StructOpt)]
    #[structopt(about = "Reset configuration properties to their defaults")]
    pub struct Reset {
        /// The config property to reset. Use 'all' to reset the entire config.
        pub property: String,
    }

    pub fn config(args: Args) -> Result<()> {
        let Args { action } = args;
        match action {
            Action::Get(action) => {
                let Get { pointer } = action;
                let config = super::display(pointer)?;
                println!("{}", config)
            }
            Action::Set(action) => {
                let Set { pointer, value } = action;
                super::set(pointer, value)?;
            }
            Action::Reset(action) => {
                let Reset { property } = action;
                super::reset(property)?;
            }
            Action::Dirs => {
                let config = util::dirs::config(false)?.display().to_string();
                let plugins = util::dirs::plugins(false)?.display().to_string();
                println!("config: {}\nplugins: {}", config, plugins);
            }
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path() -> Result<()> {
        let path = path()?;
        assert!(path.starts_with(std::env::temp_dir()));
        Ok(())
    }
}
