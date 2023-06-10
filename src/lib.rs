/*!
 * # PNGme binary crate
 *
 * Main crate that executes the PNGme application.
 * Users are able to invoke the following operations:
 * - encode
 * - decode
 * - remove
 * - print
 *
 */

/// Args crate used as module
mod args;
/// Chunk crate used as module
mod chunk;
/// Chunk type crate used as module
mod chunk_type;
/// Commands crate used as module
mod commands;
/// PNG crate used as module
mod png;

use args::PngMeArgs;
use clap::Parser;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum ConfigError {
    ArgumentParsing(clap::Error),
    CommandExecution(commands::CommandError),
}

impl From<clap::Error> for ConfigError {
    fn from(item: clap::Error) -> ConfigError {
        ConfigError::ArgumentParsing(item)
    }
}

impl From<commands::CommandError> for ConfigError {
    fn from(item: commands::CommandError) -> ConfigError {
        ConfigError::CommandExecution(item)
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigError::ArgumentParsing(ref err) => write!(f, "{}", err),
            ConfigError::CommandExecution(ref err) => write!(f, "Command error: {}", err),
        }
    }
}

impl error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            ConfigError::ArgumentParsing(ref err) => Some(err),
            ConfigError::CommandExecution(ref err) => Some(err),
        }
    }
}

/**
 * Structure to hold the application configuration. Configuration is extracted from command line arguments
 */
pub struct Config {
    args: PngMeArgs,
}

impl Config {
    pub fn build(args: impl Iterator<Item = String>) -> Result<Config, ConfigError> {
        Ok(Config {
            args: PngMeArgs::try_parse_from(args)?,
        })
    }
}

/**
 * Interface to the executable crate to run the application based on the configuration
 */
pub fn run(config: Config) -> Result<(), ConfigError> {
    match config.args {
        PngMeArgs::Encode(args) => commands::encode(args)?,
        PngMeArgs::Decode(args) => commands::decode(args)?,
        PngMeArgs::Remove(args) => commands::remove(args)?,
        PngMeArgs::Print(args) => commands::print_chunks(args)?,
    };

    Ok(())
}
