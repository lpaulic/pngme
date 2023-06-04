mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use args::PngMeArgs;
use clap::Parser;

#[derive(Debug)]
pub enum ConfigError {
    UnsupportedArgument,
    ArgumentParsing(clap::Error),
}

impl From<clap::Error> for ConfigError {
    fn from(item: clap::Error) -> ConfigError {
        ConfigError::ArgumentParsing(item)
    }
}

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

pub fn run(config: Config) -> Result<(), ConfigError> {
    match config.args {
        PngMeArgs::Encode(encode) => {
            println!("File path: {}", encode.file_path.display());
            println!("Chunk type: {}", encode.chunk_type);
            println!("Message: {}", encode.message);
            println!(
                "Output file path: {}",
                match encode.output_file_path {
                    Some(p) => p.display().to_string(),
                    None => "n/a".to_string(),
                }
            );
        }
        PngMeArgs::Decode(decode) => {
            println!("File path: {}", decode.file_path.display());
            println!("Chunk type: {}", decode.chunk_type);
        }
        PngMeArgs::Remove(remove) => {
            println!("File path: {}", remove.file_path.display());
            println!("Chunk type: {}", remove.chunk_type);
        }
        PngMeArgs::Print(print) => {
            println!("File path: {}", print.file_path.display());
        }
    };

    Ok(())
}
