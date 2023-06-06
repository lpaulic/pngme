/*
    TODO: general cleanup
        - error name from ConfigError to something more appropriate
        - move config to another crate/module
        - combine args and commands
*/

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use args::PngMeArgs;
use clap::Parser;

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

pub struct Config {
    args: PngMeArgs,
}

impl Config {
    pub fn build(args: impl Iterator<Item = String>) -> Result<Config, ConfigError> {
        // TODO: handle usage/error message and help, error message is now a ugly JSON
        Ok(Config {
            args: PngMeArgs::try_parse_from(args)?,
        })
    }
}

pub fn run(config: Config) -> Result<(), ConfigError> {
    match config.args {
        PngMeArgs::Encode(args) => commands::encode(args)?,
        PngMeArgs::Decode(args) => commands::decode(args)?,
        PngMeArgs::Remove(args) => commands::remove(args)?,
        PngMeArgs::Print(args) => commands::print_chunks(args)?,
    };

    Ok(())
}
