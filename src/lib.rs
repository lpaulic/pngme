use std::error::Error;

// TODO: check should use be first or mod
mod args;
mod chunk_type;
mod chunks;
mod commands;
mod png;

pub struct Config {
    pub message: String,
    pub png_file: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // skipping program name on first position

        let message = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a message to embed in PNG"),
        };

        let png_file = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a PNG file path"),
        };

        Ok(Config { message, png_file })
    }
}

pub fn run(_config: Config) -> Result<(), Box<dyn Error>> {
    todo!()
}
